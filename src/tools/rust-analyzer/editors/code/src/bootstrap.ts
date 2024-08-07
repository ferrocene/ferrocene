import * as vscode from "vscode";
import * as os from "os";
import type { Config } from "./config";
import { type Env, log } from "./util";
import type { PersistentState } from "./persistent_state";
import { exec, spawnSync } from "child_process";

export async function bootstrap(
    context: vscode.ExtensionContext,
    config: Config,
    state: PersistentState,
): Promise<string> {
    const path = await getServer(context, config, state);
    if (!path) {
        throw new Error(
            "rust-analyzer Language Server is not available. " +
                "Please, ensure its [proper installation](https://rust-analyzer.github.io/manual.html#installation).",
        );
    }

    log.info("Using server binary at", path);

    if (!isValidExecutable(path, config.serverExtraEnv)) {
        throw new Error(
            `Failed to execute ${path} --version.` + config.serverPath
                ? `\`config.server.path\` or \`config.serverPath\` has been set explicitly.\
            Consider removing this config or making a valid server binary available at that path.`
                : "",
        );
    }

    return path;
}
async function getServer(
    context: vscode.ExtensionContext,
    config: Config,
    state: PersistentState,
): Promise<string | undefined> {
    const packageJson: {
        version: string;
        releaseTag: string | null;
        enableProposedApi: boolean | undefined;
    } = context.extension.packageJSON;

    // check if the server path is configured explicitly
    const explicitPath = process.env["__RA_LSP_SERVER_DEBUG"] ?? config.serverPath;
    if (explicitPath) {
        if (explicitPath.startsWith("~/")) {
            return os.homedir() + explicitPath.slice("~".length);
        }
        return explicitPath;
    }
    if (packageJson.releaseTag === null) return "rust-analyzer";

    if (vscode.workspace.workspaceFolders?.length === 1) {
        // otherwise check if there is a toolchain override for the current vscode workspace
        // and if the toolchain of this override has a rust-analyzer component
        // if so, use the rust-analyzer component
        const toolchainTomlExists = await fileExists(
            vscode.Uri.joinPath(vscode.workspace.workspaceFolders[0]!.uri, "rust-toolchain.toml"),
        );
        if (toolchainTomlExists) {
            const res = spawnSync("rustup", ["which", "rust-analyzer"], {
                encoding: "utf8",
                env: { ...process.env },
                cwd: vscode.workspace.workspaceFolders[0]!.uri.fsPath,
            });
            if (!res.error && res.status === 0) {
                return res.stdout.trim();
            }
        }
    }

    // finally, use the bundled one
    const ext = process.platform === "win32" ? ".exe" : "";
    const bundled = vscode.Uri.joinPath(context.extensionUri, "server", `rust-analyzer${ext}`);
    const bundledExists = await fileExists(bundled);
    if (bundledExists) {
        let server = bundled;
        if (await isNixOs()) {
            server = await getNixOsServer(
                context.globalStorageUri,
                packageJson.version,
                ext,
                state,
                bundled,
                server,
            );
            await state.updateServerVersion(packageJson.version);
        }
        return server.fsPath;
    }

    await vscode.window.showErrorMessage(
        "Unfortunately we don't ship binaries for your platform yet. " +
            "You need to manually clone the rust-analyzer repository and " +
            "run `cargo xtask install --server` to build the language server from sources. " +
            "If you feel that your platform should be supported, please create an issue " +
            "about that [here](https://github.com/rust-lang/rust-analyzer/issues) and we " +
            "will consider it.",
    );
    return undefined;
}

async function fileExists(uri: vscode.Uri) {
    return await vscode.workspace.fs.stat(uri).then(
        () => true,
        () => false,
    );
}

export function isValidExecutable(path: string, extraEnv: Env): boolean {
    log.debug("Checking availability of a binary at", path);

    const res = spawnSync(path, ["--version"], {
        encoding: "utf8",
        env: { ...process.env, ...extraEnv },
    });

    if (res.error) {
        log.warn(path, "--version:", res);
    } else {
        log.info(path, "--version:", res);
    }
    return res.status === 0;
}

async function getNixOsServer(
    globalStorageUri: vscode.Uri,
    version: string,
    ext: string,
    state: PersistentState,
    bundled: vscode.Uri,
    server: vscode.Uri,
) {
    await vscode.workspace.fs.createDirectory(globalStorageUri).then();
    const dest = vscode.Uri.joinPath(globalStorageUri, `rust-analyzer${ext}`);
    let exists = await vscode.workspace.fs.stat(dest).then(
        () => true,
        () => false,
    );
    if (exists && version !== state.serverVersion) {
        await vscode.workspace.fs.delete(dest);
        exists = false;
    }
    if (!exists) {
        await vscode.workspace.fs.copy(bundled, dest);
        await patchelf(dest);
    }
    server = dest;
    return server;
}

async function isNixOs(): Promise<boolean> {
    try {
        const contents = (
            await vscode.workspace.fs.readFile(vscode.Uri.file("/etc/os-release"))
        ).toString();
        const idString = contents.split("\n").find((a) => a.startsWith("ID=")) || "ID=linux";
        return idString.indexOf("nixos") !== -1;
    } catch {
        return false;
    }
}

async function patchelf(dest: vscode.Uri): Promise<void> {
    await vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: "Patching rust-analyzer for NixOS",
        },
        async (progress, _) => {
            const expression = `
            {srcStr, pkgs ? import <nixpkgs> {}}:
                pkgs.stdenv.mkDerivation {
                    name = "rust-analyzer";
                    src = /. + srcStr;
                    phases = [ "installPhase" "fixupPhase" ];
                    installPhase = "cp $src $out";
                    fixupPhase = ''
                    chmod 755 $out
                    patchelf --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" $out
                    '';
                }
            `;
            const origFile = vscode.Uri.file(dest.fsPath + "-orig");
            await vscode.workspace.fs.rename(dest, origFile, { overwrite: true });
            try {
                progress.report({ message: "Patching executable", increment: 20 });
                await new Promise((resolve, reject) => {
                    const handle = exec(
                        `nix-build -E - --argstr srcStr '${origFile.fsPath}' -o '${dest.fsPath}'`,
                        (err, stdout, stderr) => {
                            if (err != null) {
                                reject(Error(stderr));
                            } else {
                                resolve(stdout);
                            }
                        },
                    );
                    handle.stdin?.write(expression);
                    handle.stdin?.end();
                });
            } finally {
                await vscode.workspace.fs.delete(origFile);
            }
        },
    );
}

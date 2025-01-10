#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

import html
import json
import sys


SUPPORTED_METRICS_FORMAT_VERSION = 1


def generate_report(data):
    if data["format_version"] != SUPPORTED_METRICS_FORMAT_VERSION:
        print(
            f"error: metrics format version {data['format_version']} is not supported",
            file=sys.stderr,
        )
        exit(1)

    print(
        """
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>Resource utilization report</title>
                <style>
                    body { font-size: 0.8em }
                    table { border-collapse: collapse }
                    td, th { padding: 0.6em 1em }
                    td { border-top: 1px solid #ccc }
                    td.data { text-align: right }
                    .quick { color: #aaa }
                    span.spacer { display: inline-block; width: 1.5rem }
                </style>
            </head>
            <body>
                <h1>Resource utilization report</h1>
                <p>Hint: hover over a type to see its debug representation.</p>
        """
    )

    for i, invocation in enumerate(data["invocations"]):
        print(
            f"""
                <div class="invocation">
                    <h2>Invocation #{i + 1}</h2>
                    <p>Total invocation duration: {invocation["duration_including_children_sec"]:.1f}s</p>
                    <table>
                        <tr>
                            <th>Type</th>
                            <th>Duration</th>
                            <th>CPU usage</th>
                        </tr>
            """
        )
        for child in invocation["children"]:
            generate_step_row(data, child, 0)
        print("</table></div>")

    print(
        f"""
                    <h2>System statistics</h2>
                    <p>CPU model: {data["system_stats"]["cpu_model"]}</p>
                    <p>CPU threads count: {data["system_stats"]["cpu_threads_count"]}</p>
                    <p>Available RAM: {format_bytes(data["system_stats"]["memory_total_bytes"])}</p>
                </body>
            </html>
        """
    )


def generate_step_row(data, step, depth):
    if step["kind"] == "test_suite":
        # Don't include test results
        return
    elif step["kind"] != "rustbuild_step":
        print(f"error: unsupported step kind: {step['kind']}", file=sys.stderr)
        exit(1)

    cpu_usage = (
        step["system_stats"]["cpu_utilization_percent"]
        / data["system_stats"]["cpu_threads_count"]
    )

    print(
        f"""
            <tr title="{html.escape(step["debug_repr"])}" {'class="quick"' if step['duration_excluding_children_sec'] < 1 else ''}>
                <td>
                    {'<span class="spacer"></span>' * depth}{step["type"]}
                </td>
                <td class="data">{step["duration_excluding_children_sec"]:.1f}s</td>
                <td class="data">{cpu_usage:.1f}%</td>
            </tr>
        """
    )

    for child in step["children"]:
        generate_step_row(data, child, depth + 1)


def format_bytes(num):
    units = ["B", "KB", "MB", "GB", "TB"]

    iterations = 0
    while num > 1024:
        num /= 1024
        iterations += 1

    return f"{num:.1f} {units[iterations]}"


def main():
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <build-metrics>", file=sys.stderr)
        exit(1)

    with open(sys.argv[1]) as f:
        data = json.load(f)

    generate_report(data)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import avh_api
import json
import uuid
import pandas
import os
import subprocess
from fabric import Connection
import sys
import time
from threading import Event
from pprint import pprint
from time import sleep
from avh_api.api import arm_api
from avh_api.model.instance_create_options import InstanceCreateOptions


def arguments():
    parser = argparse.ArgumentParser(
        prog="avh.py",
        description="Manage various aspects of ARM AVH",
    )
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    show_parser = subparsers.add_parser("show", help="Show AVH instances")

    start_parser = subparsers.add_parser("start", help="Start an AVH instance")
    start_parser.add_argument('instance', help='The AVH instance name')

    stop_parser = subparsers.add_parser("stop", help="Stop an AVH instance")
    stop_parser.add_argument('instance', help='The AVH instance name')

    forward_parser = subparsers.add_parser("forward", help="Port forward to an AVH instance")
    forward_parser.add_argument('instance', help='The AVH instance name')
    forward_parser.add_argument('local_port', help='The local port to forward to the remote')
    forward_parser.add_argument('remote_port', help='The remote port to forward to the local')

    shell_parser = subparsers.add_parser("shell", help="Shell to an AVH instance")
    shell_parser.add_argument('instance', help='The AVH instance name')

    put_parser = subparsers.add_parser("put", help="Put a file on an AVH instance")
    put_parser.add_argument('instance', help='The AVH instance name')
    put_parser.add_argument('source', help='The source path')
    put_parser.add_argument('destination', help='The destination path')

    get_parser = subparsers.add_parser("get", help="Get a file from an AVH instance")
    get_parser.add_argument('instance', help='The AVH instance name')
    get_parser.add_argument('source', help='The source path')
    get_parser.add_argument('destination', help='The destination path')

    run_parser = subparsers.add_parser("run", help="Run a command on an AVH instance")
    run_parser.add_argument('instance', help='The AVH instance name')
    run_parser.add_argument('command', help='The command to run')
    run_parser.add_argument('--forward', nargs=2, help='Forward from source to destination')

    return parser.parse_args()


def run():
    args = arguments()
    try:
        avh_token = os.environ["AVH_API_KEY"]
    except:
        print("Set AVH_API_KEY environment to an ARM AVH token")
        exit(1)

    # match added in 3.10
    if args.subcommand == "show":
        subcommand_show(avh_token, args)
    elif args.subcommand == "start":
        subcommand_start(avh_token, args)
    elif args.subcommand == "stop":
        subcommand_stop(avh_token, args)
    elif args.subcommand == "forward":
        subcommand_forward(avh_token, args)
    elif args.subcommand == "create":
        subcommand_create(avh_token, args)
    elif args.subcommand == "destroy":
        subcommand_destroy(avh_token, args)
    elif args.subcommand == "shell":
        subcommand_shell(avh_token, args)
    elif args.subcommand == "put":
        subcommand_put(avh_token, args)
    elif args.subcommand == "get":
        subcommand_get(avh_token, args)
    elif args.subcommand == "run":
        subcommand_run(avh_token, args)
    else:
        print("Unhandled subcommand")
        exit(1)

def show_instances(instances):
    frames = []
    for instance in instances:
        frame = {
            "id": instance.id,
            "name": instance.name,
            "state": instance.state,
            "flavor": instance.flavor,
            "flavorName": instance.flavorName
        }
        frames.append(frame)
    dataframes = pandas.DataFrame.from_records(frames)
    print(dataframes)

def get_instance(avh_client, uuid_or_name):
    """
    Turns the UUID or name into a Instance, so names and UUIDs can both be used.

    This is cached in the client, don't use this for polling state.
    """
    try:
        instances = avh_client.v1_get_instances(id=uuid_or_name)
    except:
        instances = avh_client.v1_get_instances(name=uuid_or_name)
    return instances[0]

def get_instance_state(avh_client, uuid):
    """
    Turns the UUID or name into a Instance, so names and UUIDs can both be used
    """
    return avh_client.v1_get_instance_state(uuid).value

def subcommand_show(avh_token, args):
    """
    Show AVH instances
    """
    avh_client = build_avh_client(avh_token)
    
    api_response = avh_client.v1_get_instances()

    show_instances(api_response)
    return

def subcommand_start(avh_token, args):
    """
    Start an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    show_instances([instance])

    try:
        api_response = avh_client.v1_start_instance(instance.id)
    except avh_api.exceptions.ApiException as e:
        if e.reason == "Conflict":
            print(f"{args.instance} is already started")
            exit(0)
        else:
            print(f"{e}")
            exit(1)

    print(f"Starting {instance.name}!")

    done = False
    poll_count = 0
    poll_limit = 25
    while not done:
        instance_state = get_instance_state(avh_client, instance.id)
        if instance_state == "on":
            break
        else:
            print(f"...{instance.name} state \"{instance_state}\" != \"on\" ({poll_count}/{poll_limit})...")
            sleep(2)
            poll_count += 1

    print(f"Started {instance.name}!")
    return

def get_ssh_client(avh_client, instance_id):
    api_response = avh_client.v1_get_instance_quick_connect_command(instance_id)

    split_response = api_response.split(" ")
    assert split_response[0] == "ssh"
    assert split_response[1] == "-J"
    jump_user_and_host = split_response[2]
    assert "@" in jump_user_and_host
    [jump_user, jump_host] = jump_user_and_host.split("@")

    user_and_host = split_response[3]
    assert "@" in user_and_host
    [user, host] = user_and_host.split("@")

    try:
        jump_client = Connection(jump_host, user=jump_user)
    except Exception as e:
        print(f"Failed to connect to jump host: {e}")
        sys.exit(1)


    try:
        ssh_client = Connection(
            host,
            user="user",
            gateway=jump_client,
            connect_kwargs={"password": "password"}
        )
    except Exception as e:
        print(f"Failed to connect to destination host: {e}")
        sys.exit(1)  
    return ssh_client

def subcommand_forward(avh_token, args):
    """
    Port forward to an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    api_response = avh_client.v1_get_instance_quick_connect_command(instance.id)
    ssh_client = get_ssh_client(avh_client, instance.id)

    try:
        print("Connected! Beginning port forward...")
        with ssh_client.forward_local(int(args.local_port), remote_port=int(args.remote_port)):
            while True:
                time.sleep(1000)
    except KeyboardInterrupt:
        print("Port forwarding stopped.")
    return


def subcommand_get(avh_token, args):
    """
    Copy from an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    source = args.source
    destination = args.destination
    ssh_client = get_ssh_client(avh_client, instance.id)

    print(f"Connected! Getting from {source} on {instance.name} to {destination}")
    ssh_client.get(source, destination, preserve_mode=True)
    return

def subcommand_put(avh_token, args):
    """
    Copy to an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    source = args.source
    destination = args.destination
    ssh_client = get_ssh_client(avh_client, instance.id)

    print(f"Connected! Putting to {source} on {instance.name} from {destination}")
    ssh_client.put(source, destination, preserve_mode=True)
    return

def subcommand_run(avh_token, args):
    """
    Copy to an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    command = args.command
    ssh_client = get_ssh_client(avh_client, instance.id)

    if args.forward:
        source_port = int(args.forward[0])
        destination_port = int(args.forward[1])
        print(f"Connected! Running to `{command}` on {instance.name} with forward from {source_port} to {destination_port}")
        with ssh_client.forward_local(source_port, remote_port=destination_port):
            ssh_client.run(command)
            try:
                while True:
                    time.sleep(1000)
            except KeyboardInterrupt:
                print("Port forwarding stopped.")
    else:
        print(f"Connected! Running to `{command}` on {instance.name}")
        ssh_client.run(command)
    return


def subcommand_shell(avh_token, args):
    """
    Shell to an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    
    api_response = avh_client.v1_get_instance_quick_connect_command(instance.id)

    split_response = api_response.split(" ")
    assert split_response[0] == "ssh"
    assert split_response[1] == "-J"
    gateway_user_and_host = split_response[2]
    assert "@" in gateway_user_and_host

    user_and_host = split_response[3]
    assert "@" in user_and_host

    full_command = split_response
    full_command_text = " ".join(full_command)

    print(f"Running `{full_command_text}`")
    subprocess.run(full_command)
    return

def subcommand_stop(avh_token, args):
    """
    Stop an AVH instance
    """
    avh_client = build_avh_client(avh_token)
    instance = get_instance(avh_client, args.instance)
    show_instances([instance])

    api_response = avh_client.v1_stop_instance(instance.id)

    print(f"Stopping {instance.name}!")

    done = False
    poll_count = 0
    poll_limit = 25
    while not done:
        instance_state = get_instance_state(avh_client, instance.id)
        if instance_state != "off":
            break
        else:
            print(f"({poll_count}/{poll_limit}) {instance.name} state \"{instance_state}\" != \"off\"...")
            sleep(2)
            poll_count += 1

    print(f"Stopped {instance.name}!")
    return

def build_avh_client(avh_token):
    configuration = avh_api.Configuration(
        access_token = avh_token
    )
    api_client = avh_api.ApiClient(configuration)
    return arm_api.ArmApi(api_client)

if __name__ == "__main__":
    run()

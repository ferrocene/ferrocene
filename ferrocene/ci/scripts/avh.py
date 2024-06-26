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

    stop_parser = subparsers.add_parser("forward", help="Port forward to an AVH instance")
    stop_parser.add_argument('instance', help='The AVH instance name')
    stop_parser.add_argument('local_port', help='The local port to forward to the remote')
    stop_parser.add_argument('remote_port', help='The remote port to forward to the local')

    # create_parser = subparsers.add_parser("create", help="Create an AVH instance")
    # create_parser.add_argument('instance', help='The AVH instance name')

    # destroy_parser = subparsers.add_parser("destroy", help="Create an AVH instance")
    # destroy_parser.add_argument('instance', help='The AVH instance name')

    return parser.parse_args()


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

    api_response = avh_client.v1_start_instance(instance.id)

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

def subcommand_forward(avh_token, args):
    """
    Port forward to an AVH instance
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

    host = user_and_host.split("@")[1]
    port_forwarding_arg = f"{args.local_port}:{host}:{args.remote_port}"

    full_command = split_response + ["-L", port_forwarding_arg]
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


def run():
    args = arguments()
    try:
        avh_token = os.environ["AVH_TOKEN"]
    except:
        print("Set AVH_TOKEN environment to an ARM AVH token")
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
    else:
        print(f"Unknown command {args.subcommand}")

if __name__ == "__main__":
    run()

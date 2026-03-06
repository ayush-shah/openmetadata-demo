#!/usr/bin/env python3
"""Hard-delete soft-deleted table entities via the OpenMetadata REST API.

Dry-run by default. Pass --execute to actually delete.
"""
import argparse
import os
import sys

import requests


def parse_args():
    parser = argparse.ArgumentParser(description="Hard-delete soft-deleted OM tables.")
    parser.add_argument("--host", default="http://localhost:8585", help="OM server base URL")
    parser.add_argument("--token", default=os.environ.get("OM_TOKEN"), help="JWT Bearer token")
    parser.add_argument("--page-size", type=int, default=100, help="Search page size")
    parser.add_argument("--execute", action="store_true", help="Actually delete (default: dry-run)")
    return parser.parse_args()


def fetch_deleted_tables(host, headers, page_size):
    """Yield (id, fqn) for every soft-deleted table via the tables REST API."""
    after = None
    while True:
        params = {"include": "deleted", "limit": page_size}
        if after:
            params["after"] = after

        resp = requests.get(
            f"{host}/api/v1/tables",
            headers=headers,
            params=params,
            timeout=30,
        )
        if not resp.ok:
            print(f"[ERROR] List failed: {resp.status_code} {resp.text}")
            sys.exit(1)

        body = resp.json()
        for table in body.get("data", []):
            yield table.get("id"), table.get("fullyQualifiedName")

        after = body.get("paging", {}).get("after")
        if not after:
            break


def main():
    args = parse_args()

    if not args.token:
        print("[ERROR] No token provided. Use --token or set OM_TOKEN.")
        sys.exit(1)

    headers = {"Authorization": f"Bearer {args.token}"}
    dry_run = not args.execute
    count = 0

    for table_id, fqn in fetch_deleted_tables(args.host, headers, args.page_size):
        count += 1
        if dry_run:
            print(f"[DRY-RUN] Would delete: {fqn} ({table_id})")
        else:
            resp = requests.delete(
                f"{args.host}/api/v1/tables/{table_id}",
                headers=headers,
                params={"hardDelete": "true", "recursive": "false"},
                timeout=30,
            )
            if resp.ok:
                print(f"[DELETED] {fqn} ({table_id})")
            else:
                print(f"[ERROR] {fqn}: {resp.status_code} {resp.text}")

    qualifier = "would be " if dry_run else ""
    print(f"\nDone. {count} tables {qualifier}deleted.")


if __name__ == "__main__":
    main()

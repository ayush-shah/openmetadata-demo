#  Copyright 2024 Collate
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#  http://www.apache.org/licenses/LICENSE-2.0
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.

"""
API Lineage CICD Demo
=====================

Demonstrates how to use the OpenMetadata Python SDK 2.0 to:
  1. Create two API REST services (Orders API, Fulfillment API) from YAML definitions
  2. Register API Collections and Endpoints under each service
  3. Add lineage edges between endpoints across services

Each step reads from a declarative YAML file so this script can be
integrated into a CI/CD pipeline.

Usage:
    pip install "openmetadata-ingestion~=1.6.0"
    python api_lineage_cicd.py
"""
from pathlib import Path
from typing import Optional

import yaml

from metadata.sdk import configure, APICollections, APIEndpoints
from metadata.sdk.api.lineage import Lineage

from metadata.generated.schema.api.lineage.addLineage import AddLineageRequest
from metadata.generated.schema.api.services.createApiService import (
    CreateApiServiceRequest,
)
from metadata.generated.schema.api.data.createAPICollection import (
    CreateAPICollectionRequest,
)
from metadata.generated.schema.api.data.createAPIEndpoint import (
    CreateAPIEndpointRequest,
)
from metadata.generated.schema.entity.data.apiEndpoint import (
    ApiRequestMethod,
    APIEndpoint,
)
from metadata.generated.schema.entity.services.apiService import (
    ApiServiceType,
    ApiService,
)
from metadata.generated.schema.type.apiSchema import APISchema
from metadata.generated.schema.type.basic import (
    FullyQualifiedEntityName,
    SqlFunction,
    SqlQuery,
)
from metadata.generated.schema.type.entityLineage import (
    ColumnLineage,
    EntitiesEdge,
    LineageDetails,
)
from metadata.generated.schema.type.entityReference import EntityReference
from metadata.generated.schema.type.schema import DataTypeTopic, FieldName, SchemaType, FieldModel

OM_HOST = "http://localhost:8585/api"
OM_JWT_TOKEN = (
    "eyJraWQiOiJHYjM4OWEtOWY3Ni1nZGpzLWE5MmotMDI0MmJrOTQzNTYiLCJ0eXAiOiJKV1QiLCJh"
    "bGciOiJSUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImlzQm90IjpmYWxzZSwiaXNzIjoib3Blbi1tZXRh"
    "ZGF0YS5vcmciLCJpYXQiOjE2NjM5Mzg0NjIsImVtYWlsIjoiYWRtaW5Ab3Blbm1ldGFkYXRhLm9y"
    "ZyJ9.tS8um_5DKu7HgzGBzS1VTA5uUjKWOCU0B_j08WXBiEC0mr0zNREkqVfwFDD-d24HlNEbrqio"
    "LsBuFRiwIWKc1m_ZlVQbG7P36RUxhuv2vbSp80FKyNM-Tj93FDzq91jsyNmsQhyNv_fNr3TXfzzSPj"
    "Ht8Go0FMMP66weoKMgW2PbXlhVKwEuXUHyakLLzewm9UMeQaEiRzhiTMU3UkLXcKbYEJJvfNFcLwSl"
    "9W8JCO_l0Yj3ud-qt_nQYEZwqW6u5nfdQllN133iikV4fM5QZsMCnm8Rq1mvLR0y9bmJiD7fwM1tmJ"
    "791TUWqmKaTnP49U493VanKpUAfzIiOiIbhg"
)


# ------------------------------------------------------------------ #
# Helpers
# ------------------------------------------------------------------ #

def load_yaml(filename: str) -> dict:
    path = Path(__file__).parent / filename
    with path.open() as f:
        return yaml.safe_load(f)


def build_api_schema(raw: Optional[dict]) -> Optional[APISchema]:
    if not raw:
        return None
    schema_fields = raw.get("schemaFields") or []
    return APISchema(
        schemaType=SchemaType(raw.get("schemaType", "JSON")),
        schemaFields=[
            FieldModel(
                name=FieldName(f["name"]),
                dataType=DataTypeTopic(f.get("dataType", "STRING")),
                description=f.get("description"),
                children=[
                    FieldModel(
                        name=FieldName(c["name"]),
                        dataType=DataTypeTopic(c.get("dataType", "STRING")),
                        description=c.get("description"),
                    )
                    for c in f.get("children", [])
                ] if f.get("children") else None,
            )
            for f in schema_fields
        ] if schema_fields else None,
    )


# ------------------------------------------------------------------ #
# Step 1 — Create an API Service from YAML
# ------------------------------------------------------------------ #

def create_api_service(raw: dict) -> ApiService:
    """Create or update an API Service and its collections/endpoints."""
    from metadata.sdk import client as get_client

    ometa = get_client().ometa

    service = ometa.create_or_update(
        CreateApiServiceRequest(
            name=raw["serviceName"],
            serviceType=ApiServiceType(raw["serviceType"]),
            displayName=raw.get("displayName"),
            description=raw.get("description"),
        )
    )
    print(f"  Service: {service.fullyQualifiedName}")

    for coll_raw in raw.get("collections", []):
        collection = APICollections.create(
            CreateAPICollectionRequest(
                name=coll_raw["name"],
                displayName=coll_raw.get("displayName"),
                description=coll_raw.get("description"),
                endpointURL=coll_raw.get("endpointURL"),
                service=service.fullyQualifiedName,
            )
        )
        print(f"    Collection: {collection.fullyQualifiedName}")

        for ep_raw in coll_raw.get("endpoints", []):
            endpoint = APIEndpoints.create(
                CreateAPIEndpointRequest(
                    name=ep_raw["name"],
                    displayName=ep_raw.get("displayName"),
                    description=ep_raw.get("description"),
                    apiCollection=collection.fullyQualifiedName,
                    endpointURL=ep_raw["endpointURL"],
                    requestMethod=ApiRequestMethod(
                        ep_raw.get("requestMethod", "GET")
                    ),
                    requestSchema=build_api_schema(ep_raw.get("requestSchema")),
                    responseSchema=build_api_schema(ep_raw.get("responseSchema")),
                )
            )
            print(f"      Endpoint: {endpoint.fullyQualifiedName}")

    return service


# ------------------------------------------------------------------ #
# Step 2 — Create lineage edges from YAML
# ------------------------------------------------------------------ #

def build_column_lineage(raw_columns: Optional[list]) -> Optional[list[ColumnLineage]]:
    """Build column-level lineage with transformation functions."""
    if not raw_columns:
        return None
    return [
        ColumnLineage(
            fromColumns=[
                FullyQualifiedEntityName(col) for col in cl["fromColumns"]
            ],
            toColumn=FullyQualifiedEntityName(cl["toColumn"]),
            function=SqlFunction(cl["function"]) if cl.get("function") else None,
        )
        for cl in raw_columns
    ]


def create_lineage(raw: dict) -> None:
    """Read lineage edges from YAML and register them in OpenMetadata."""
    for edge in raw.get("edges", []):
        from_ref = edge["from"]
        to_ref = edge["to"]

        from_fqn = f"{from_ref['service']}.{from_ref['collection']}.{from_ref['endpoint']}"
        to_fqn = f"{to_ref['service']}.{to_ref['collection']}.{to_ref['endpoint']}"

        from_entity = APIEndpoints.retrieve_by_name(from_fqn)
        to_entity = APIEndpoints.retrieve_by_name(to_fqn)

        columns_lineage = build_column_lineage(edge.get("columnLineage"))

        request = AddLineageRequest(
            edge=EntitiesEdge(
                fromEntity=EntityReference(
                    id=from_entity.id,
                    type="apiEndpoint",
                ),
                toEntity=EntityReference(
                    id=to_entity.id,
                    type="apiEndpoint",
                ),
                lineageDetails=LineageDetails(
                    sqlQuery=SqlQuery(edge["sqlQuery"]) if edge.get("sqlQuery") else None,
                    columnsLineage=columns_lineage,
                    description=edge.get("description"),
                ),
            )
        )
        Lineage.add_lineage_request(request)

        print(f"  Lineage: {from_fqn} → {to_fqn}")
        for cl in columns_lineage or []:
            src = ", ".join(str(c) for c in (cl.fromColumns or []))
            print(f"    {src} --[{cl.function}]--> {cl.toColumn}")


# ------------------------------------------------------------------ #
# Step 3 — Verify lineage
# ------------------------------------------------------------------ #

def verify_lineage() -> None:
    """Fetch lineage back to confirm it was created."""
    endpoint = APIEndpoints.retrieve_by_name(
        "fulfillment-api.shipments.createShipment"
    )
    lineage = Lineage.get_entity_lineage(
        entity_type=APIEndpoint,
        entity_id=endpoint.id,
        upstream_depth=2,
        downstream_depth=2,
    )
    if lineage:
        upstream = lineage.upstreamEdges or []
        downstream = lineage.downstreamEdges or []
        print(f"  createShipment — upstream edges: {len(upstream)}, downstream edges: {len(downstream)}")
    else:
        print("  No lineage found (unexpected)")


# ------------------------------------------------------------------ #
# Main
# ------------------------------------------------------------------ #

def run() -> None:
    configure(host=OM_HOST, jwt_token=OM_JWT_TOKEN)

    print("\n1. Creating Orders API...")
    orders_raw = load_yaml("orders_api.yaml")
    create_api_service(orders_raw)

    print("\n2. Creating Fulfillment API...")
    fulfillment_raw = load_yaml("fulfillment_api.yaml")
    create_api_service(fulfillment_raw)

    print("\n3. Adding lineage edges...")
    lineage_raw = load_yaml("lineage.yaml")
    create_lineage(lineage_raw)

    print("\n4. Verifying lineage...")
    verify_lineage()

    print("\nDone ✅")


if __name__ == "__main__":
    run()

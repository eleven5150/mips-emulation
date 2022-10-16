import json
from pathlib import Path
from typing import Dict, Any, List

import jsonschema
import dacite


class DataclassDaciteMixinAbstract:
    __keys_to_clean: List[str] = [
        '$schema',
        '$ref',
        '$comment',
        '$id',
    ]

    @classmethod
    def _clean_dict(cls, data: Dict[str, Any]) -> Dict[str, Any]:
        result = data.copy()
        for key in cls.__keys_to_clean:
            if key not in result:
                continue

            del result[key]

        return result


class DataclassDaciteStrictMixin(DataclassDaciteMixinAbstract):
    @classmethod
    def from_dict(cls, data: Dict[str, Any]):
        return dacite.from_dict(
            data_class=cls,
            data=cls._clean_dict(data),
            config=dacite.Config(
                check_types=True,
                strict=True
            )
        )


def validate_json(data, schema):
    jsonschema.validate(instance=data, schema=schema)


def get_parsed_config_generic(cls, data_path: Path, schema_path: Path):
    with open(data_path, 'r') as raw_file:
        data = json.load(raw_file)

    with open(schema_path, 'r') as raw_file:
        schema = json.load(raw_file)

    validate_json(data, schema)
    return cls.from_dict(data)


from .pyrs_reverse_geocoder import reverse_geocode as rust_reverse_geocode
from .pyrs_reverse_geocoder import bulk_reverse_geocode as rust_bulk_reverse_geocode

import typing

class Location:
    def __init__(self, _rust_tuple):
        """ A object representing a location """
        self._rust_location_struct = _rust_tuple
        if len(self._rust_location_struct) != 4:
            raise ValueError
        

    @property
    def city(self):
        return self._rust_location_struct[0]

    @property
    def state(self):
        return self._rust_location_struct[1]

    @property
    def country(self):
        return self._rust_location_struct[2]

    @property
    def formatted(self):
        return self._rust_location_struct[3]


def bulk_reverse_geocode(list_of_coordinates: typing.List[typing.Tuple[int, int]]) -> typing.List[Location]:
    _items = rust_bulk_reverse_geocode(list_of_coordinates)
    return [Location(x) for x in _items]

def reverse_geocode(latitude: int, longitude: int):
    return Location(rust_reverse_geocode(latitude, longitude))

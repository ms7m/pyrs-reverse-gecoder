# Python-Rust Reverse Geocoder
***

This is a simple, no-dependency alternative package to 

- https://github.com/thampiman/reverse-geocoder
- https://github.com/richardpenman/reverse_geocode/

The difference being that the entire logic is handled in Rust. This greatly improves CPU performance, speed and overall size. (No longer depends on Numpy or Scipy).

This library exposes [the reverse_geocoder crate](https://crates.io/crates/reverse_geocoder) to python.


## Single Location
```python

>>> from pyrs_reverse_geocoder.reverse_geocoder import reverse_geocode
>>> p = reverse_geocode(36.0797958, -79.4934861)
>>> 
>>> p.formatted
'North Carolina, Alamance County, US'
>>> 
```

## Bulk Locations

```python
>>> from pyrs_reverse_geocoder.reverse_geocoder import bulk_reverse_geocode
>>> 
>>> coords = [(36.41287, -89.0405), (40.71494, -74.05077), (35.9273343, -78.8617589), (39.7764626, -85.8010164), (36.08757, -79.77241), (36.04728, -79.84211), (32.57573, -97.08872), (36.0983198, -79.5241177), (35.93994, -83.98838), (35.95891, -80.03308)]
>>> 
>>> 
>>> bulk = bulk_reverse_geocode(coords)
>>> 
>>> 
>>> for location in bulk:
...     print(location.formatted)
... 
Tennessee, Obion County, US
New Jersey, Hudson County, US
North Carolina, Durham County, US
Indiana, Hancock County, US
North Carolina, Guilford County, US
North Carolina, Guilford County, US
Texas, Tarrant County, US
North Carolina, Alamance County, US
Tennessee, Knox County, US
North Carolina, Guilford County, US
>>> 
```
# Installing

There is no wheels currently (Coming soon). You'll need to have rust toolchain installed.

- Clone this repository
- Create virtual python environment (targeting python version 3.6+)
- Activate environment
- Install dependencies in ``requirements.txt``.
- ``make build``
- Install the ``.whl`` file in target/wheels

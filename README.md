# Elasticsearch importer example
This piece of code will give you example how to import data to elastic search.

## Elasticsearch Map
Eventhough ES will be able to receive and process dynamic / not defined scheme but it is considered best practice to have defined map.

```json
{
    "mappings" : {
        "name" : {
            
        },
        "address" : {
            
        }
    }
}
```

# To do
* Give more structure
* Get data from db / sqlite
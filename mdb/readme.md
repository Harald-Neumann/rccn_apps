To add the generated mdb to yamcs:

1. Place the output of the python script into and xml file in `.../yamcs-instance/src/main/yamcs/mdb`
2. Add it in `.../yamcs-instance/src/main/yamcs/etc/yamcs.rccn_usr.yaml` e.g.

```yaml
...
mdb:
    - type: "xtce"
        spec: "mdb/your_file.xml"
    ...
...
```

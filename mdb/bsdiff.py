# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *
#from rustgen import *

service = System("BsDiffService")
service_type_id = 131

base_cmd = Command(
    system=service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": service_type_id},
)


createCommand = Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 1},
    name="CreatePatch",
    arguments=[
        StringArgument(
            name="file_1",
            encoding=StringEncoding(64),
        ),
        StringArgument(
            name="file_2",
            encoding=StringEncoding(64),
        ),
        StringArgument(
            name="output-file",
            short_description="The output file to write the patched source to. By default it will be the same as the source file", 
            encoding=StringEncoding(64),
        ),
    ]
)

patchCommand = Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 2},
    name="ApplyPatch",
    arguments=[
                StringArgument(
            name="source-file",
            short_description="The source file to apply the patch to",
            encoding=StringEncoding(64),
        ),
        StringArgument(
            name="patch-file",
            short_description="The patch file to apply to the source file",
            encoding=StringEncoding(64),
        ),
        StringArgument(
            name="output-file",
            short_description="The output file to write the patched source to. By default it will be the same as the source file", 
            encoding=StringEncoding(64),
        ),
    ]
)



print(service.dumps())
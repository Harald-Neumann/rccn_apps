# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *
#from rustgen import *

service = System("BsDiffService")
service_type_id = 131

string_length = 8

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
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
        StringArgument(
            name="file_2",
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
        StringArgument(
            name="output-file",
            short_description="The output file to write the patched source to. By default it will be the same as the source file", 
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
        EnumeratedArgument( # This mainly serves as an exercise to see how to use enumerated arguments and how to send tm as a response
            name="send-patch",
            short_description="Wether the content of the patch file should be send to the ground",
            choices=[
                (1, "Yes"),
                (2, "No"),
            ],
            default="Yes",
            encoding=IntegerEncoding(8),
        )
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
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
        StringArgument(
            name="patch-file",
            short_description="The patch file to apply to the source file",
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
        StringArgument(
            name="output-file",
            short_description="The output file to write the patched source to. By default it will be the same as the source file", 
            max_length=string_length,
            encoding=StringEncoding(string_length * 8),
        ),
    ]
)



print(service.dumps())
import amaranth
import amaranth_soc

import lambdasoc

from xml.dom import minidom
from xml.etree import ElementTree
from xml.etree.ElementTree import Element, SubElement, Comment, tostring

from os import path


# Generate a svd file for the given MinervaSoC
def build(soc: lambdasoc.soc.cpu.CPUSoC, vendor="amaranth-soc", name="soc", build_dir="build/", description=None):
    device = _build_section_device(soc, vendor, name, description)

    # <peripherals />
    peripherals = SubElement(device, "peripherals")

    # no resources wut?
    #print("resources: ", soc.memory_map.resources())
    #for resource, resource_name, (start, stop) in soc.memory_map.resources():
    #    print(resource_name)

    window: amaranth_soc.memory.MemoryMap
    for window, (start, stop, ratio) in soc.memory_map.windows():
        if window.name not in ["uart", "timer"]:
            continue

        peripheral = _build_section_peripheral(peripherals, window, start, stop, ratio)
        registers = SubElement(peripheral, "registers")

        resource_info: amaranth_soc.memory.ResourceInfo
        for resource_info in window.all_resources():

            register = _build_section_register(registers, resource_info)
            fields = SubElement(register, "fields")

            for ast in resource_info.resource:
                if isinstance(ast, amaranth.hdl.ast.Signal):
                    #print("signal: ", str(ast))
                    pass
                elif isinstance(ast, amaranth.hdl.ast.Slice):
                    #print("slice: ", str(ast))
                    pass
                else:
                    print("unhandled amaranth ast element: ", type(slice))

            # TODO can we go lower?
            field = _build_section_field(fields, resource_info)

    # <vendorExtensions />
    vendorExtensions = SubElement(device, "vendorExtensions")

    memoryRegions = SubElement(vendorExtensions, "memoryRegions")

    window: amaranth_soc.memory.MemoryMap
    for window, (start, stop, ratio) in soc.memory_map.windows():
        if window.name not in ["bootrom", "sram", "scratchpad"]:
            continue

        memoryRegion = SubElement(memoryRegions, "memoryRegion")
        el = SubElement(memoryRegion, "name")
        el.text = window.name.upper()
        el = SubElement(memoryRegion, "baseAddress")
        el.text = "0x{:08x}".format(start)
        el = SubElement(memoryRegion, "size")
        el.text = "0x{:08x}".format(stop - start)

    constants = SubElement(vendorExtensions, "constants")  # TODO


    # dump
    print("\n")
    output = ElementTree.tostring(device, 'utf-8')
    output = minidom.parseString(output)
    #output = output.toprettyxml(indent="  ")
    output = output.toprettyxml(indent="  ", encoding="utf-8")
    #print(output)
    with open(path.join(build_dir, name + ".svd"), "w") as f:
        f.write(str(output.decode("utf-8")))
        f.close()


def _build_section_device(soc: lambdasoc.soc.cpu.CPUSoC, vendor, name, description):
    device = Element("device")
    device.set("schemaVersion", "1.1")
    device.set("xmlns:xs", "http://www.w3.org/2001/XMLSchema-instance")
    device.set("xs:noNamespaceSchemaLocation", "CMSIS-SVD.xsd")
    el = SubElement(device, "vendor")
    el.text = vendor
    el = SubElement(device, "name")
    el.text = name.upper()
    el = SubElement(device, "description")
    if description is None:
        el.text = "TODO device.description"
    else:
        el.text = description
    el = SubElement(device, "addressUnitBits")
    el.text = "8"          # TODO
    el = SubElement(device, "width")
    el.text = "32"         # TODO
    el = SubElement(device, "size")
    el.text = "32"         # TODO
    el = SubElement(device, "access")
    el.text = "read-write"
    el = SubElement(device, "resetValue")
    el.text = "0x00000000" # TODO
    el = SubElement(device, "resetMask")
    el.text = "0xFFFFFFFF" # TODO

    return device


def _build_section_peripheral(peripherals: Element, window, start, stop, ratio):
    peripheral = SubElement(peripherals, "peripheral")
    el = SubElement(peripheral, "name")
    el.text = window.name.upper()
    el = SubElement(peripheral, "groupName")
    el.text = window.name.upper()
    el = SubElement(peripheral, "baseAddress")
    el.text = "0x{:08x}".format(start)

    addressBlock = SubElement(peripheral, "addressBlock")
    el = SubElement(addressBlock, "offset")
    el.text = "0" # TODO
    el = SubElement(addressBlock, "size")     # TODO
    el.text = "0x{:02x}".format(stop - start) # TODO
    el = SubElement(addressBlock, "usage")
    el.text = "registers" # TODO

    return peripheral


def _build_section_register(registers: Element, resource_info: amaranth_soc.memory.ResourceInfo):
    register = SubElement(registers, "register")
    el = SubElement(register, "name")
    el.text = "_".join(resource_info.name)
    el = SubElement(register, "description")
    el.text = "TODO register.description"
    el = SubElement(register, "addressOffset")
    el.text = "0x{:04x}".format(resource_info.start)
    el = SubElement(register, "size")
    el.text = "{:d}".format((resource_info.end - resource_info.start) * 8) # TODO
    el = SubElement(register, "resetValue")
    el.text = "0x00"

    return register


def _build_section_field(fields: Element, resource_info: amaranth_soc.memory.ResourceInfo):
    field =  SubElement(fields, "field")
    el = SubElement(field, "name")
    el.text = "_".join(resource_info.name)
    el = SubElement(field, "description")
    el.text = "TODO field.description"
    el = SubElement(field, "bitRange")
    el.text = "[31:0]" # TODO

    return field

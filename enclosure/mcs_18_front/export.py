#!/usr/bin/env python3

import sys
sys.path.append('/usr/lib/freecad-python3/lib/')

try:
    import FreeCAD
except ValueError:
    print('failed  to load FreeCAD')
else:
    doc = FreeCAD.openDocument('mcs_18_front.csg')
    obj = doc.getObject('Box')
    Import.export([obj], 'test.step')


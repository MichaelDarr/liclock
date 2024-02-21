bodyHeight = 4.4;
edgeLength = 18.2;

module shell_boundary() {
    color("black")
        linear_extrude(height=bodyHeight, center=false)
            square([edgeLength, edgeLength], center=true);
}

inscribeAtCorner = 2.5;
inscribeAtCenterEdge = 1.8;
inscribedEdgeLength = edgeLength - (inscribeAtCorner + inscribeAtCenterEdge);
bottomThickness = 0.3;

module shell() {
    difference() {
        shell_boundary();
        translate([0, 0, bottomThickness])
            color("black")
                linear_extrude(height=bodyHeight, center=false)
                    square([inscribedEdgeLength, inscribedEdgeLength], center=true);
    }
}

diaphragmInset = 1;
diaphragmHeight = 0.45;
diaphragmRadius = 40;

module diaphragm() {
    intersection() {
        shell_boundary();
        translate([0, 0, ((diaphragmRadius * -1) + bodyHeight) - (diaphragmInset - diaphragmHeight)])
            color("#bf180f")
                sphere(diaphragmRadius, $fn=80);
    }

    translate([0, 0, bottomThickness])
        color("#d41b11")
            linear_extrude(height=bodyHeight-bottomThickness-diaphragmInset, center=false)
                square([inscribedEdgeLength, inscribedEdgeLength], center=true);
}


pinDiameter = 1;
pinHeight = 5.8;
pinHousingHeight = 3.2;
pinHousingDiameter = 4.4;

module pin(x=0) {
    translate([x, 0, 0]) {
        rotate([0, 180, 0]) {
            color("#101010")
                linear_extrude(height=pinHousingHeight, center=false)
                    circle(d=pinHousingDiameter, $fn=200);
            translate([0, 0, pinHousingHeight])
                color("gainsboro")
                    linear_extrude(height=pinHeight, center=false)
                        circle(d=pinDiameter, $fn=100);
        }
    }
}


centerNubDiameter = 3.2;
centerWallThickness = 0.7;

module centerNub(x=0) {
    rotate([0, 180, 0])
        color("#101010")
            difference() {
                linear_extrude(height=pinHousingHeight, center=false)
                    circle(d=centerNubDiameter, $fn=100);
                linear_extrude(height=pinHousingHeight, center=false)
                    circle(d=(centerNubDiameter - (centerWallThickness * 2)), $fn=100);
            }
}

outerPinSpacing = 8.5;
innerPinSpacing = outerPinSpacing-pinDiameter;
centerToPinCenter = innerPinSpacing/2;
nubIsolation = 4.8;

module pins() {
    centerNub();
    difference() {
        union() {
            pin(centerToPinCenter);
            pin(centerToPinCenter * -1);
        }
        translate([0, 0, -0.001])
            rotate([0, 180, 0]) 
                color("#101010")
                    linear_extrude(height=pinHousingHeight+0.002, center=false)
                        square([nubIsolation, nubIsolation], center=true);
    }
}


edgeToKey = 1.6;
keyWidth = 2.2;

module key() {
    translate([(edgeLength/2) - edgeToKey, (keyWidth/2)*-1, 0])
        rotate([0, 180, 0])
            color("#101010")
                linear_extrude(height=pinHousingHeight, center=false)
                    square([keyWidth, keyWidth], center=false);
}


shell();
diaphragm();
pins();
key();

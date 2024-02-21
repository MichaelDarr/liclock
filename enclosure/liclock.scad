// tip-to-tip clock length
Length = 174;
// clock edge to switch cutout
SwitchBezel = 7.4;
// side length for cherry mx cutouts
MXCutout=14;

// side length for mcs 18 front cutout
MCS18FrontCutout=18.6;

module cutout_mx(x=0, y=0, z=0, lipHeight=1.5, lipDepth=2) {
    // Cherry MX switch model
    translate([x, y, z+13.1])
        %import("switch_mx.stl");

    // Geometry
    difference() {
        children();
        translate([x, y, z]) {
            // Lip cutout
            rotate([0, 180, 0])
                linear_extrude(height=lipHeight, center=false)
                    square([MXCutout, MXCutout], center=true);
            // Primary cutout
            translate([0, 0, lipHeight * -1])
                rotate([0, 180, 0])
                    linear_extrude(height=8.3-lipHeight, center=false)
                        square([MXCutout+lipDepth, MXCutout+lipDepth], center=true);
         }
    }
}

module cutout_mcs_18_front(x=0, y=0, z=0, lipHeight=1.6) {
    // Cherry MX switch model
    translate([0, 0, lipHeight*-1])
    %import("mcs_18_front/mcs_18_front.stl");

    difference() {
        children();
        // Lip cutout
        rotate([0, 180, 0])
            linear_extrude(height=lipHeight, center=false)
                square([MCS18FrontCutout, MCS18FrontCutout], center=true);
        // Primary cutout
        translate([0, 0, (lipHeight * -1) + 0.001])
            rotate([0, 180, 0]) {
                union() {
                    // symetric octagonal cutout
                    intersection() {
                        rotate([0, 0, 45])
                            linear_extrude(height=9-lipHeight, center=false)
                                square([MCS18FrontCutout-4, MCS18FrontCutout-4], center=true);
                        linear_extrude(height=9-lipHeight, center=false)
                            square([MCS18FrontCutout-6, MCS18FrontCutout-6], center=true);
                    }
                    // directional key cutout
                    translate([-7, 0, 0])
                        linear_extrude(height=9-lipHeight, center=false)
                            square([2, 3], center=true);
                }
            }
    }
}

module cutout_lcd(x=0, y=0, z=0) {
    translate([x, y, z])
        rotate([45, 0, 0])
            rotate([0, 0, 90])
                %import("LCD-S401C71TR.stl");

    difference() {
        children();
        translate([x, y, z])
            rotate([45, 0, 0])
                rotate([0, 0, 90])
                    linear_extrude(height=10, center=false, scale=[1.5,1.15])
                        square([32.4, 65.4], center=true);
    }
}


module cutout_power_switch(x=0, y=0, z=0) {
    translate([x, y, z])
        rotate([180,90,0])
            rotate([0,0,180])
                scale([25.4,25.4,25.4])
                    %import("CWSA11AAN1H.stl");

    difference() {
        children();
    translate([x, y, z])
        rotate([180,90,0])
            rotate([0,0,180])
                union() {
                    scale([25.4,25.4,25.4])
                        %import("CWSA11AAN1H.stl");
                    translate([0, 1, 0])
                        linear_extrude(height=21, center=true)
                            square([15, 2], center=true);
                    translate([0, -1, 0])
                        linear_extrude(height=20, center=true)
                            square([13, 2], center=true);
                    translate([0, -8, 0])
                        linear_extrude(height=20, center=true)
                            square([12.4, 12], center=true);
                }
    }
}

centerToTip = Length/2;
centerToSwitch = centerToTip-SwitchBezel-(MXCutout/2);

translate([0, 0, 54]) {
    // Top Face
    cutout_mx(x=centerToSwitch) cutout_mx(x=centerToSwitch*-1) cutout_mcs_18_front()
        rotate([0, 180, 0])
            linear_extrude(height=5, center=false)
                square([Length, 45], center=true);

    cutout_lcd(x=42, y=-37.2, z=-28.8) cutout_lcd(x=-42, y=-37.2, z=-28.8) cutout_power_switch(x=-73, y=20.8, z=-44)
        difference() {
            translate([0, 22.5, 0])
                rotate([-90,0,0])
                    rotate([0,90,0])
                        linear_extrude(height=Length, center=true)
                            polygon(
                                points=[
                                    [0,0],[45,0],[90,45],[90,54],[0,54],
                                    [10,5],[40,5],[80,45],[80,54],[10,54]
                                ],
                                paths=[[0,1,2,3,4], [5,6,7,8,9]]
                            );
            rotate([0, 180, 0])
                linear_extrude(height=5, center=false)
                    square([Length, 45], center=true);
        }
}

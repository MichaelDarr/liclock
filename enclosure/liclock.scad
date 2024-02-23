// tip-to-tip clock length
Length = 178;
// clock edge to switch cutout
SwitchBezel = 9.4;
// side length for cherry mx cutouts
MXCutout=14;

// side length for mcs 18 front cutout
MCS18FrontCutout=18;

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
            // Above-lip cutout
            linear_extrude(height=4.4-lipHeight, center=false)
                square([MXCutout+lipDepth, MXCutout+lipDepth], center=true);
        }
    }
}

module cutout_mcs_18_front(x=0, y=0, z=0, lipHeight=4.4) {
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
                    translate([-6.7, 0, 0])
                        linear_extrude(height=9-lipHeight, center=false)
                            square([1.85, 2.55], center=true);
                }
            }
    }
}

module cutout_lcd(x=0, y=0, z=0) {
    translate([x, y+1.6, z-1.6])
        rotate([45, 0, 0])
            rotate([0, 0, 90])
                %import("LCD-S401C71TR.stl");

    difference() {
        children();
        translate([x, y, z])
            rotate([45, 0, 0])
                rotate([0, 0, 90])
                linear_extrude(height=7.5, center=false, scale=[1.75,1.2], twist=0, slices=20)
                        square([22.86, 62.23], center=true);
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

module cutout_mounting_hole(x=0, y=0, z=0) {
    difference() {
        children();
        translate([x, y, z]) {
            rotate([45, 0, 0])
                rotate([0, 180, 0])
                    linear_extrude(height=12, center=false)
                        circle(d=3.66, $fn=36);
            rotate([45, 0, 0])
                rotate([0, 180, 0])
                    linear_extrude(height=2.2, center=false)
                        circle(d=6.8, $fn=36);
        }
    }
}

centerToTip = Length/2; // 87
centerToSwitch = centerToTip-SwitchBezel-(MXCutout/2); // 87 - 7.4 - (14/2)

translate([0, 0, 49]) {
    // Top Face
    translate([0, -7.8, -5])
        cutout_mx(x=centerToSwitch, z=-2.8) cutout_mx(x=centerToSwitch*-1, z=-2.8) cutout_mcs_18_front()
            rotate([0, 180, 0])
                linear_extrude(height=7.8, center=false)
                    square([Length, 26], center=true);

    // Body
    cutout_lcd(x=42, y=-36.7, z=-29.3) cutout_lcd(x=-42, y=-36.7, z=-29.3)
    cutout_power_switch(x=-74, y=5.2, z=-38)
    cutout_mounting_hole(y=-41.9, z=-24.2) cutout_mounting_hole(x=84, y=-41.9, z=-24.2) cutout_mounting_hole(x=-84, y=-41.9, z=-24.2)
        difference() {
            translate([0, 22.5, 0])
                rotate([-90,0,0])
                    rotate([0,90,0])
                        linear_extrude(height=Length, center=true)
                            polygon(
                                points=[
                                    [15.4,5],[45,5],[83,43],[83,49],[15.4,49],
                                    [22,10],[40,10],[73,43],[73,49],[22,49]
                                ],
                                paths=[[0,1,2,3,4], [5,6,7,8,9]]
                            );
            translate([0, -7.8, -5])
                rotate([0, 180, 0])
                    linear_extrude(height=7.8, center=false)
                        square([Length, 26], center=true);
        }
}

// tip-to-tip clock length
Length = 178;
// clock edge to switch cutout
SwitchBezel = 9.4;
// clock edge to foot cutout
FootBezel = 4;
// side length for cherry mx cutouts
MXCutout=14;

// side length for mcs 18 front cutout
MCS18FrontCutout=17;

module cutout_mx(x=0, y=0, z=0, lipHeight=1.5, lipDepth=2) {
    // Switch model
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
                    linear_extrude(height=8.3-lipHeight, center=false, scale=[0.9, 1.29])
                        square([MXCutout, MXCutout+0.56], center=true);

            // Above-lip cutout
            translate([0, 0, -0.01])
                linear_extrude(height=2.4-lipHeight, center=false, scale=1.327)
                    square([MXCutout+lipDepth, MXCutout+lipDepth], center=true);
            translate([0, 0, 0.85])
                linear_extrude(height=4.8-lipHeight, center=false, scale=1.1)
                    square([MXCutout+lipDepth+3, MXCutout+lipDepth+3], center=true);
        }
    }
}

module cutout_mcs_18_front(x=0, y=0, z=0, lipHeight=4.4) {
    // Switch model
    translate([x, y, z+(lipHeight*-1)])
        %import("mcs_18_front/mcs_18_front.stl");

    difference() {
        children();
        translate([x, y, z]) {
            // Lip cutout
            rotate([0, 180, 0])
                linear_extrude(height=lipHeight, center=false)
                    square([MCS18FrontCutout+0.4, MCS18FrontCutout+0.4], center=true);
            // Primary cutout
            translate([0, 0, (lipHeight * -1) + 0.001])
                rotate([0, 180, 0]) {
                    union() {
                        // symetric octagonal cutout
                        intersection() {
                            rotate([0, 0, 45])
                                linear_extrude(height=9-lipHeight, center=false)
                                    square([MCS18FrontCutout-3.6, MCS18FrontCutout-3.6], center=true);
                            linear_extrude(height=9-lipHeight, center=false)
                                square([MCS18FrontCutout-5.4, MCS18FrontCutout-5.4], center=true);
                        }
                        // directional key cutout
                        translate([-6.7, 0, 0])
                            linear_extrude(height=9-lipHeight, center=false)
                                square([1.85, 2.55], center=true);
                    }
                }
        }
    }
}

module cutout_lcd(x=0, y=0, z=0) {
    translate([x, y+1.6, z-1.6])
        rotate([45, 0, 0])
            rotate([0, 0, 90])
                %import("../pcb/Clock-Parts.3dshapes/LCD-S401C71TR.stl");

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
                    %import("../pcb/Clock-Parts.3dshapes/CWSA11AAN1H.stl");

    difference() {
        children();
        translate([x, y, z])
            rotate([180,90,0])
                rotate([0,0,180])
                    union() {
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
                        circle(d=4, $fn=16);
            rotate([45, 0, 0])
                rotate([0, 180, 0])
                    linear_extrude(height=2.2, center=false)
                        circle(d=7, $fn=56);
        }
    }
}

// https://www.essentracomponents.com/en-us/p/push-fit-rivet-feet/fsr-1?indexed=true
// 1.2 - 2.1
fsr1_panel_thickness = 1.5;
// 3.6-3.7
fsr1_hole_diameter = 3.7;
// guess
fsr1_insert_diameter = 5.5;
// rivet = 5.1
fsr1_insert_depth = 6;

// Potential alternative:
// https://www.heyco.com/Nylon_PVC_Hardware/pdf/Push-In-Bumpers-2.pdf
// Mouting Hole Diameter: 6.4mm/0.25"
// Max Panel Thickness: 3.2mm/0.125"

module cutout_fsr1(x=0, y=0, z=0) {
    translate([x, y, z-4.05])
        rotate([-90, 0, 0])
            %import("fsr_1.stl");

    difference() {
        children();
        translate([x, y, z]) {
            rotate([-90, 0, 0]) {
                rotate([90, 0, 0])
                    linear_extrude(height=fsr1_insert_depth, center=false)
                        circle(d=fsr1_hole_diameter, $fn=22);
                translate([0, fsr1_panel_thickness * -1, 0])
                    rotate([90, 0, 0])
                        linear_extrude(height=fsr1_insert_depth - fsr1_panel_thickness, center=false)
                            circle(d=fsr1_insert_diameter, $fn=22);
            }
        }
    }
}

centerToTip = Length/2; // 87
centerToSwitch = centerToTip-SwitchBezel-(MXCutout/2); // 87 - 7.4 - (14/2)

centerToFooter = centerToTip - FootBezel;

translate([0, 0, 49]) {
    // Top Face
    translate([0, -7.8, -5])
        cutout_mx(x=centerToSwitch, z=-2.4) cutout_mx(x=centerToSwitch*-1, z=-2.4) cutout_mcs_18_front(z=0.38)
            rotate([0, 180, 0])
                linear_extrude(height=7.2, center=false)
                    square([Length, 26], center=true);

    // Body
    cutout_lcd(x=42, y=-36.7, z=-29.3) cutout_lcd(x=-42, y=-36.7, z=-29.3)
    cutout_power_switch(x=-70, y=5.2, z=-38)
    cutout_mounting_hole(y=-41.9, z=-24.2) cutout_mounting_hole(x=84, y=-41.9, z=-24.2) cutout_mounting_hole(x=-84, y=-41.9, z=-24.2)
    cutout_fsr1(x=centerToFooter, y=3.8, z=-49.01) cutout_fsr1(x=centerToFooter, y=-55.2, z=-49.01)
    cutout_fsr1(x=centerToFooter * -1, y=3.8, z=-49.01) cutout_fsr1(x=centerToFooter * -1, y=-55.2, z=-49.01)
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
                    linear_extrude(height=7.2, center=false)
                        square([Length, 26], center=true);
        }
}

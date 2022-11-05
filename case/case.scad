FLOPPY_SIZE = [101.6,25.4];
N_FLOPPIES = 6;

Z_PCB = 10 + FLOPPY_SIZE.y * 6 + 3;
X_BORDER = (125 - FLOPPY_SIZE.x)/2;

module kosmoPanel2d(w) {
  difference(){
    square([w,200]);
    for (p = [
      [5, 5],
      [5, 195],
      [w-5, 5],
      [w-5, 195],
    ]) {
      translate(p) circle(d=3);
    }
  }
}

module ports2d() {
  // ports on the pcb, measured form the bottom of the PCB
  translate([10.892, 1.6+.3]) square([12.319, 10.6]);
  translate([37, 1.6+9.75]) circle(d=18);
  translate([83.496, 1.6+6.5]) circle(d=11.2);
}

module frontPanel2d() {
  difference(){
    kosmoPanel2d(125);
    translate([X_BORDER, 10])square([FLOPPY_SIZE.x, FLOPPY_SIZE.y * N_FLOPPIES]);
    translate([X_BORDER, Z_PCB]) ports2d();
  }
}


module side2d() {
  difference(){
    hull(){
      translate([90,15])circle(r=5);
      translate([85,10 + N_FLOPPIES * FLOPPY_SIZE.y])circle(r=10);
      translate([0, 180])circle(r=10);
      translate([0, 20])circle(r=10);
    }
    for(i = [0:N_FLOPPIES - 1]){
      for(x = [25, 85]){
        translate([x, 5 + 10 + i * FLOPPY_SIZE.y]) circle(d=3);
      }
    }
    translate([-11,0])square([11,200]);
  }
}

module pcbMount2d() {
  difference(){
    union(){
      translate([X_BORDER, 0, 0])square([8.8,95]);
      translate([X_BORDER+FLOPPY_SIZE.x-8.8, 0, 0])square([8.8,95]);
    }
    for(pos = [
      [125/2+46, 4+3],
      [125/2-46, 4+3],
      [125/2+46, 3+92 - 4],
      [125/2-46, 3+92 - 4],
    ]){
      #translate(pos)circle(d=2.8);
    }
  }
}

module case(){
  rotate([90,0,0])translate([0,0,-3])linear_extrude(3)frontPanel2d();
  translate([X_BORDER-3,0,0])rotate([90,0,90])linear_extrude(3)side2d();
  translate([125-X_BORDER,0,0])rotate([90,0,90])linear_extrude(3)side2d();
  translate([0,0,Z_PCB-3])linear_extrude(3)pcbMount2d();
}

case();

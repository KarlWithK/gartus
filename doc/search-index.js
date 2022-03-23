var searchIndex = JSON.parse('{\
"lib":{"doc":"An amateur computer graphics engine made in Rust.","i":[[0,"graphics","lib","This module hosts all the needed struts to playing around …",null,null],[0,"display","lib::graphics","Includes the [Canvas] and [Pixel] struts, which come …",null,null],[3,"Pixel","lib::graphics::display","A computer pixel struct is represented by its red, green, …",null,null],[12,"red","","The first byte that represents red light",0,null],[12,"green","","The second byte that represents green light",0,null],[12,"blue","","The final byte that represents blue light",0,null],[11,"new","","Returns a pixel that will be used in [Canvas]",0,[[["u8",15]]]],[3,"Canvas","","An art [Canvas] / computer screen is represented here.",null,null],[12,"upper_left_system","","A boolean that will determine where \\\"(0, 0)\\\" - the start …",1,null],[12,"line","","A [Pixel] that represents the color that will be used to …",1,null],[11,"new","","Returns a new blank [Canvas] to be drawn on.",1,[[["u32",15],["u8",15]]]],[11,"new_with_bg","","Returns a new [Canvas] to be drawn on with a specific …",1,[[["u32",15],["pixel",3],["u8",15]]]],[11,"get_width","","Returns the width of a [Canvas]",1,[[],["u32",15]]],[11,"get_height","","Returns the height of a [Canvas].",1,[[],["u32",15]]],[11,"set_line_pixel","","Sets the color of the drawing line to a different color …",1,[[["pixel",3]]]],[11,"set_line_color","","Sets the color of the drawing line to a different color …",1,[[["u8",15]]]],[11,"get_pixel","","Returns a [Pixel] given a (X, Y) coordinate pair that …",1,[[["i32",15]],["pixel",3]]],[11,"plot","","Plots new_color to the (X, Y) coordinate pair …",1,[[["pixel",3],["i32",15]]]],[11,"clear_canvas","","Clears the [Canvas] to Pixel::default()",1,[[]]],[11,"fill_color","","Fills the entire [Canvas] with one [Pixel]",1,[[["pixel",3]]]],[11,"save_ascii","","Saves the current state of an image as an ascii ppm file.",1,[[["str",15]],["result",6]]],[11,"save_binary","","Saves the current state of an image as a binary ppm file.",1,[[["str",15]],["result",6]]],[11,"save_extension","","Saves the current state of an image to any extension.",1,[[["str",15]],["result",6]]],[11,"display","","Display the current state of the [Canvas].",1,[[],["result",6]]],[0,"draw","lib::graphics","Hosts all the functions needed to start drawing onto the […",null,null],[11,"fill","lib::graphics::display","Fills in the area of a 2D figure given a random point …",1,[[["pixel",3],["i32",15]]]],[11,"fill_with_animation","","Fills in the area of a 2D figure given a random point …",1,[[["pixel",3],["str",15],["i32",15]]]],[11,"draw_lines","","Draws all lines in provided in a given [Matrix] onto the […",1,[[["matrix",3]]]],[11,"draw_lines_for_animation","","Draws all lines in provided in a given [Matrix] onto the […",1,[[["matrix",3],["str",15]]]],[11,"draw_line","","Draws a line onto the [Canvas] provided two sets of …",1,[[["pixel",3],["f64",15]]]],[0,"matrix","lib::graphics","Includes the [Matrix] struct with a surrounding mini …",null,null],[3,"Matrix","lib::graphics::matrix","A m x n Matrix is represented here",null,null],[11,"new","","Returns a new row x column [Matrix] with a vector that …",2,[[["usize",15],["f64",15],["vec",3]]]],[11,"get_num_points","","Returns the number of points (rows) currently in the […",2,[[],["usize",15]]],[11,"identity_matrix","","Returns a new N by N identity [Matrix].",2,[[["usize",15]]]],[11,"transpose","","Returns the transpose [Matrix] of self.",2,[[]]],[11,"identifize","","Makes self an identity [Matrix] if the matrix is N by N.",2,[[]]],[11,"fill","","Fills every element of self.data with a specific float.",2,[[["f64",15]]]],[11,"swap_rows","","Swaps two rows in self.data.",2,[[["usize",15]]]],[11,"get","","Returns the corresponding self.data element given a row …",2,[[["usize",15]],["f64",15]]],[11,"set","","Sets the corresponding self.data element a new value …",2,[[["usize",15],["f64",15]]]],[11,"iter_row","","Returns a iterator that iterates over a specific row.",2,[[["usize",15]]]],[11,"iter_col","","Returns a iterator that iterates over a specific column.",2,[[["usize",15]]]],[11,"iter_col_mut","","Returns a mutable iterator that iterates over a specific …",2,[[["usize",15]]]],[11,"iter_by_point","","Returns a iterator that iterates over the [Matrix]\'s …",2,[[],[["chunksexact",3],["f64",15]]]],[11,"iter_by_point_mut","","Returns a mutable iterator that iterates over the [Matrix]…",2,[[],[["f64",15],["chunksexactmut",3]]]],[11,"add_point","","Adds a new point (x, y, z) to a [Matrix].",2,[[["f64",15]]]],[11,"append_point","","Appends a point in the form of a vector to the edge […",2,[[["vec",3]]]],[11,"add_edge","","Adds a new edge to an edge [Matrix].",2,[[["f64",15]]]],[11,"add_edge_vec","","Adds a new edge in the form of a f64 vector to an edge […",2,[[["f64",15],["vec",3]]]],[11,"mult_matrix","","Returns a the result of multiplying self by another […",2,[[]]],[11,"mult_vector","","Returns the resulting vecotr when multiplying by self.",2,[[["f64",15],["vec",3]]]],[0,"transformations","lib::graphics","Hosts all the functions needed to start with 3D …",null,null],[11,"reflect_yz","lib::graphics::matrix","Returns a reflection over the yz (y) axis transformation […",2,[[]]],[11,"reflect_xz","","Returns a reflection over the xz (x) axis transformation […",2,[[]]],[11,"reflect_xy","","Returns a reflection over the xy (y) axis transformation […",2,[[]]],[11,"reflect_45","","Returns a reflection over the y=x axis transformation […",2,[[]]],[11,"reflect_neg45","","Returns a reflection over the y=-x axis transformation […",2,[[]]],[11,"reflect_origin","","Returns a reflection over the origin transformation […",2,[[]]],[11,"translate","","Returns a translation transformation [Matrix].",2,[[["f64",15]]]],[11,"scale","","Returns a dilation transformation [Matrix].",2,[[["f64",15]]]],[11,"rotate_point","","Returns a transformation [Matrix] to allow a vector to be …",2,[[["f64",15]]]],[11,"rotate_x","","Returns a rotation over the x axis transformation [Matrix]…",2,[[["f64",15]]]],[11,"rotate_y","","Returns a rotation over the y axis transformation [Matrix]…",2,[[["f64",15]]]],[11,"rotate_z","","Returns a rotation over the z axis transformation [Matrix]…",2,[[["f64",15]]]],[11,"shearing_x","","Returns a shearing over the x axis transformation [Matrix]…",2,[[["f64",15]]]],[11,"shearing_y","","Returns a shearing over the y axis transformation [Matrix]…",2,[[["f64",15]]]],[11,"shearing_z","","Returns a shearing over the z axis transformation [Matrix]…",2,[[["f64",15]]]],[0,"parser","lib","This module hosts a [Parser] that allows an image to be …",null,null],[3,"Parser","lib::parser","",null,null],[11,"new","","Returns a parser that can parse through <code>file_name</code>",3,[[["u32",15],["pixel",3],["u8",15],["str",15]]]],[11,"new_with_bg","","Returns a parser that can parse through <code>file_name</code> that …",3,[[["u32",15],["pixel",3],["u8",15],["str",15]]]],[11,"parse_file","","Parses and runs through the commands in self.file_name",3,[[]]],[0,"utils","lib","This module provides utilities that might be needed to …",null,null],[5,"animation","lib::utils","Returns a new animation given a file name prefix.",null,[[["str",15]]]],[5,"view_animation","","Open\'s a given animation using imagemagick\'s <code>animate</code>.",null,[[["str",15]]]],[11,"from","lib::graphics::display","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","lib::graphics::matrix","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"to_string","","",2,[[],["string",3]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","lib::parser","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone","lib::graphics::display","",0,[[],["pixel",3]]],[11,"clone","","",1,[[],["canvas",3]]],[11,"clone","lib::graphics::matrix","",2,[[],["matrix",3]]],[11,"default","lib::graphics::display","",0,[[],["pixel",3]]],[11,"default","","",1,[[],["canvas",3]]],[11,"default","lib::graphics::matrix","",2,[[],["matrix",3]]],[11,"default","lib::parser","",3,[[],["parser",3]]],[11,"eq","lib::graphics::display","",0,[[["pixel",3]],["bool",15]]],[11,"ne","","",0,[[["pixel",3]],["bool",15]]],[11,"eq","lib::graphics::matrix","",2,[[],["bool",15]]],[11,"fmt","lib::graphics::display","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","lib::graphics::matrix","",2,[[["formatter",3]],["result",6]]],[11,"fmt","lib::parser","",3,[[["formatter",3]],["result",6]]],[11,"fmt","lib::graphics::matrix","",2,[[["formatter",3]],["result",6]]],[11,"div","","",2,[[]]],[11,"sub","","",2,[[]]],[11,"add","","",2,[[]]],[11,"mul","","",2,[[["matrix",3]]]],[11,"mul","","",2,[[]]],[11,"add_assign","","",2,[[]]],[11,"add_assign","","",2,[[["f64",15]]]],[11,"add_assign","","",2,[[]]],[11,"add_assign","","",2,[[["f64",15],["vec",3]]]],[11,"sub_assign","","",2,[[]]],[11,"sub_assign","","",2,[[["f64",15]]]],[11,"mul_assign","","",2,[[]]],[11,"mul_assign","","",2,[[["matrix",3]]]],[11,"mul_assign","","",2,[[["f64",15]]]],[11,"div_assign","","",2,[[["f64",15]]]]],"p":[[3,"Pixel"],[3,"Canvas"],[3,"Matrix"],[3,"Parser"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);
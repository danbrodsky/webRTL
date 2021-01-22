/* AUTOMATICALLY GENERATED VERILOG-2001 SOURCE CODE.
** GENERATED BY CLASH 1.2.5. DO NOT MODIFY.
*/
`timescale 100fs/100fs
module parallax
    ( // Inputs
      input  clk // clock
    , input  rst // reset
    , input  en

      // Outputs
    , output wire [31:0] pixel
    );
  reg [31:0] result = 32'd4278190080;
  // parallax.hs:76:1-4
  wire [84:0] ws;
  wire [31:0] result_0;
  wire [4:0] c$case_scrut;
  // parallax.hs:76:1-4
  wire [3:0] v;
  reg [31:0] c$case_alt;
  // parallax.hs:76:1-4
  wire [79:0] c$ws_app_arg;
  // parallax.hs:76:1-4
  wire [255:0] xs;
  wire [95:0] result_1;
  // parallax.hs:76:1-4
  wire [15:0] c$ws_app_arg_0;
  // parallax.hs:76:1-4
  wire [255:0] c$xs_app_arg;
  // parallax.hs:(39,1)-(48,5)
  reg [32:0] hvSync = {16'b0000000000000000,16'b0000000000000000,1'b0};
  reg [32:0] c$case_alt_0;
  reg [32:0] c$case_alt_1;
  wire [271:0] c$vec;
  wire [79:0] c$vec2;
  wire [255:0] c$vec_0;
  wire [15:0] c$case_alt_0_selection;
  wire [0:0] c$case_alt_1_selection;

  // register begin
  always @(posedge clk or  posedge  rst) begin : result_register
    if ( rst) begin
      result <= 32'd4278190080;
    end else  if (en)  begin
      result <= result_0;
    end
  end
  // register end

  assign ws = {c$ws_app_arg,{1'b0,4'bxxxx}};

  assign result_0 = c$case_scrut[4:4] ? c$case_alt : 32'd4294967295;

  assign c$case_scrut = ws[85-1 -: 5];

  assign v = c$case_scrut[3:0];

  always @(*) begin
    case(v)
      4'd0 : c$case_alt = 32'd4278190080;
      4'd1 : c$case_alt = 32'd4279242768;
      4'd2 : c$case_alt = 32'd4280295456;
      4'd3 : c$case_alt = 32'd4281677109;
      4'd4 : c$case_alt = 32'd4282729797;
      4'd5 : c$case_alt = 32'd4283782485;
      4'd6 : c$case_alt = 32'd4284835173;
      4'd7 : c$case_alt = 32'd4285887861;
      4'd8 : c$case_alt = 32'd4287269514;
      4'd9 : c$case_alt = 32'd4288322202;
      4'd10 : c$case_alt = 32'd4289374890;
      4'd11 : c$case_alt = 32'd4290427578;
      4'd12 : c$case_alt = 32'd4291480266;
      4'd13 : c$case_alt = 32'd4292861919;
      4'd14 : c$case_alt = 32'd4293914607;
      default : c$case_alt = 32'd4294967295;
    endcase
  end

  // imap begin
  genvar i;
  generate
  for (i=0; i < 16; i = i + 1) begin : imap
    wire [4-1:0] map_index;
    wire [5:0] map_in;
    assign map_in = result_1[i*6+:6];
    wire [4:0] map_out;

    assign map_index = 4'd15 - i[0+:4];
    // parallax.hs:76:1-4
    wire  x;
    assign x = map_in[5:5];

    assign map_out = x ? {1'b1,map_index} : map_in[4:0];


    assign c$ws_app_arg[i*5+:5] = map_out;
  end
  endgenerate
  // imap end

  assign c$vec = {hvSync[32:17] + hvSync[16:1]
                 ,c$xs_app_arg};

  assign xs = c$vec[272-1 : 16];

  assign c$vec2 = (ws[80-1 : 0]);

  // zipWith start
  genvar i_0;
  generate
  for (i_0 = 0; i_0 < 16; i_0 = i_0 + 1) begin : zipWith
    wire  zipWith_in1;
    assign zipWith_in1 = c$ws_app_arg_0[i_0*1+:1];
    wire [4:0] zipWith_in2;
    assign zipWith_in2 = c$vec2[i_0*5+:5];
    wire [5:0] c$n;
    assign c$n = {zipWith_in1,zipWith_in2};


    assign result_1[i_0*6+:6] = c$n;
  end
  endgenerate
  // zipWith end

  // map begin
  genvar i_1;
  generate
  for (i_1=0; i_1 < 16; i_1 = i_1 + 1) begin : map
    wire [15:0] map_in_0;
    assign map_in_0 = xs[i_1*16+:16];
    wire  map_out_0;
    wire [31:0] c$app_arg;
    assign c$app_arg = map_in_0 * 16'b0000001100110011;

    assign map_out_0 = (c$app_arg[(64'sd16)]) == (1'b1);


    assign c$ws_app_arg_0[i_1*1+:1] = map_out_0;
  end
  endgenerate
  // map end

  assign c$vec_0 = (xs);

  // map begin
  genvar i_2;
  generate
  for (i_2=0; i_2 < 16; i_2 = i_2 + 1) begin : map_0
    wire [15:0] map_in_1;
    assign map_in_1 = c$vec_0[i_2*16+:16];
    wire [15:0] map_out_1;
    assign map_out_1 = map_in_1 + hvSync[16:1];


    assign c$xs_app_arg[i_2*16+:16] = map_out_1;
  end
  endgenerate
  // map end

  // register begin
  always @(posedge clk or  posedge  rst) begin : hvSync_register
    if ( rst) begin
      hvSync <= {16'b0000000000000000,16'b0000000000000000,1'b0};
    end else  if (en)  begin
      hvSync <= c$case_alt_0;
    end
  end
  // register end

  assign c$case_alt_0_selection = hvSync[16:1];

  always @(*) begin
    case(c$case_alt_0_selection)
      16'b0000110001111111 : c$case_alt_0 = {hvSync[32:17] + 16'b0000000000000001
                                            ,16'b0000000000000000
                                            ,hvSync[0:0]};
      default : c$case_alt_0 = c$case_alt_1;
    endcase
  end

  assign c$case_alt_1_selection = hvSync[0:0];

  always @(*) begin
    case(c$case_alt_1_selection)
      1'b0 : c$case_alt_1 = {hvSync[32:17]
                            ,hvSync[16:1]
                            ,1'b1};
      default : c$case_alt_1 = {hvSync[32:17]
                               ,hvSync[16:1] + 16'b0000000000000001
                               ,hvSync[0:0]};
    endcase
  end

  assign pixel = result;


endmodule

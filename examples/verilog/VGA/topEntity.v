/* AUTOMATICALLY GENERATED VERILOG-2001 SOURCE CODE.
** GENERATED BY CLASH 1.2.5. DO NOT MODIFY.
*/
`timescale 100fs/100fs
module topEntity
    ( // Inputs
      input  clk // clock
    , input  rst // reset
    , input  en
    , input [0:0] eta1

      // Outputs
    , output wire [3:0] result
    );
  reg [3:0] result_0 = 4'b0000;
  wire [3:0] c$app_arg;
  // controller.hs:(45,1)-(56,5)
  reg [22:0] hvSync = {1'b0,1'b0,1'b0,10'b0000000000,10'b0000000000};
  wire  c$case_alt;
  wire  c$app_arg_0;
  wire  c$case_alt_0;
  wire  c$app_arg_1;
  wire  c$app_arg_2;
  wire [22:0] c$case_alt_1;
  // controller.hs:31:1-11
  wire [9:0] x4;
  // controller.hs:31:1-11
  wire [9:0] x5;
  reg [22:0] c$hvSyncState_$jOut_app_arg;
  reg [22:0] c$hvSyncState_$jOut_case_alt;
  wire [9:0] c$bv;
  wire [9:0] c$hvSyncState_$jOut_app_arg_selection;
  wire [9:0] c$hvSyncState_$jOut_case_alt_selection;

  // register begin
  always @(posedge clk or  posedge  rst) begin : result_0_register
    if ( rst) begin
      result_0 <= 4'b0000;
    end else  if (en)  begin
      result_0 <= c$app_arg;
    end
  end
  // register end

  assign c$bv = ((hvSync[19:10] & 10'b1111111111) >> (64'sd6));

  assign c$app_arg = hvSync[20:20] ? (c$bv[0+:4]) : 4'b0000;

  // register begin
  always @(posedge clk or  posedge  rst) begin : hvSync_register
    if ( rst) begin
      hvSync <= {1'b0,1'b0,1'b0,10'b0000000000,10'b0000000000};
    end else  if (en)  begin
      hvSync <= c$case_alt_1;
    end
  end
  // register end

  assign c$case_alt = (hvSync[19:10] < 10'b1011110000) ? 1'b0 : 1'b1;

  assign c$app_arg_0 = (hvSync[19:10] > 10'b1010010000) ? c$case_alt : 1'b1;

  assign c$case_alt_0 = (hvSync[9:0] < 10'b0111101100) ? 1'b0 : 1'b1;

  assign c$app_arg_1 = (hvSync[9:0] > 10'b0111101010) ? c$case_alt_0 : 1'b1;

  assign c$app_arg_2 = (hvSync[19:10] < 10'b1010000000) ? (hvSync[9:0] < 10'b0111100000) : 1'b0;

  assign c$case_alt_1 = {c$app_arg_0
                        ,c$app_arg_1
                        ,c$app_arg_2
                        ,x4
                        ,x5};

  assign x4 = c$hvSyncState_$jOut_app_arg[19:10];

  assign x5 = c$hvSyncState_$jOut_app_arg[9:0];

  assign c$hvSyncState_$jOut_app_arg_selection = hvSync[19:10];

  always @(*) begin
    case(c$hvSyncState_$jOut_app_arg_selection)
      10'b1100100000 : c$hvSyncState_$jOut_app_arg = c$hvSyncState_$jOut_case_alt;
      default : c$hvSyncState_$jOut_app_arg = {hvSync[22:22]
                                              ,hvSync[21:21]
                                              ,hvSync[20:20]
                                              ,hvSync[19:10] + 10'b0000000001
                                              ,hvSync[9:0]};
    endcase
  end

  assign c$hvSyncState_$jOut_case_alt_selection = hvSync[9:0];

  always @(*) begin
    case(c$hvSyncState_$jOut_case_alt_selection)
      10'b1000001101 : c$hvSyncState_$jOut_case_alt = {hvSync[22:22]
                                                      ,hvSync[21:21]
                                                      ,hvSync[20:20]
                                                      ,10'b0000000000
                                                      ,10'b0000000000};
      default : c$hvSyncState_$jOut_case_alt = {hvSync[22:22]
                                               ,hvSync[21:21]
                                               ,hvSync[20:20]
                                               ,10'b0000000000
                                               ,hvSync[9:0] + 10'b0000000001};
    endcase
  end

  assign result = result_0;


endmodule


// Lean compiler output
// Module: Bastion.Runtime
// Imports: public import Init public meta import Init
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
uint8_t lean_uint64_dec_le(uint64_t, uint64_t);
uint64_t lean_uint64_sub(uint64_t, uint64_t);
uint64_t lean_uint64_land(uint64_t, uint64_t);
uint8_t lean_uint64_dec_eq(uint64_t, uint64_t);
uint8_t lean_uint64_dec_lt(uint64_t, uint64_t);
uint64_t lean_uint64_shift_right(uint64_t, uint64_t);
uint64_t lean_uint64_mul(uint64_t, uint64_t);
uint64_t lean_uint64_mod(uint64_t, uint64_t);
uint64_t lean_uint64_add(uint64_t, uint64_t);
uint64_t lean_uint64_shift_left(uint64_t, uint64_t);
uint64_t lean_uint64_lor(uint64_t, uint64_t);
uint64_t lean_uint64_div(uint64_t, uint64_t);
LEAN_EXPORT uint64_t bastion_abi_version(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_abiVersion___boxed(lean_object*);
LEAN_EXPORT uint8_t bastion_valid_config(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_validConfig___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_valid_limits(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_validLimits___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_can_reserve(uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_canReserve___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_reservation_status(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_reservationStatus___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_reserve_value(uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_reserveValue___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_can_release(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_canRelease___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_release_value(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_releaseValue___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_next_identifier(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextIdentifier___boxed(lean_object*);
LEAN_EXPORT uint8_t bastion_same_identity(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_sameIdentity___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_can_grant(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_canGrant___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_rights_allow(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_rightsAllow___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_authorized(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_authorized___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_restrict_rights(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_restrictRights___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_revoke_target(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_revokeTarget___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_charge(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_charge___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_clock_valid(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_clockValid___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_account(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_account___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_runnable(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_runnable___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_Bastion_Runtime_eligible(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_eligible___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_next_slot(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextSlot___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_next_cursor(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextCursor___boxed(lean_object*);
LEAN_EXPORT uint64_t bastion_deadline(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_deadline___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_valid_user_return(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_validUserReturn___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_user_flags(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_userFlags___boxed(lean_object*);
LEAN_EXPORT uint64_t bastion_supervisor_entry(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_supervisorEntry___boxed(lean_object*);
LEAN_EXPORT uint64_t bastion_user_page_entry(uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_userPageEntry___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_syscall_opcode(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_syscallOpcode___boxed(lean_object*);
LEAN_EXPORT uint8_t bastion_net_frame_len(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_netFrameLen___boxed(lean_object*);
LEAN_EXPORT uint8_t bastion_net_ipv4(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_netIpv4___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_net_udp(uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_netUdp___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t bastion_net_budget(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_netBudget___boxed(lean_object*);
LEAN_EXPORT uint8_t bastion_relay_ingress(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_relayIngress___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_field_share(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldShare___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t l_Bastion_Runtime_fieldInv(uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldInv___boxed(lean_object*);
LEAN_EXPORT uint64_t l_Bastion_Runtime_weight(uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_weight___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_field_reconstruct(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldReconstruct___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_unique_step(uint64_t, uint64_t, uint64_t);
LEAN_EXPORT lean_object* l_Bastion_Runtime_uniqueStep___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint64_t bastion_abi_version(uint64_t v_x_1_){
_start:
{
uint64_t v___x_2_; 
v___x_2_ = 1ULL;
return v___x_2_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_abiVersion___boxed(lean_object* v_x_3_){
_start:
{
uint64_t v_x_8__boxed_4_; uint64_t v_res_5_; lean_object* v_r_6_; 
v_x_8__boxed_4_ = lean_unbox_uint64(v_x_3_);
lean_dec_ref(v_x_3_);
v_res_5_ = bastion_abi_version(v_x_8__boxed_4_);
v_r_6_ = lean_box_uint64(v_res_5_);
return v_r_6_;
}
}
LEAN_EXPORT uint8_t bastion_valid_config(uint64_t v_period_7_, uint64_t v_quantum_8_){
_start:
{
uint8_t v___x_9_; uint64_t v___x_10_; uint8_t v___x_11_; 
v___x_9_ = lean_uint64_dec_le(v_quantum_8_, v_period_7_);
v___x_10_ = 0ULL;
v___x_11_ = lean_uint64_dec_lt(v___x_10_, v_period_7_);
if (v___x_11_ == 0)
{
if (v___x_11_ == 0)
{
return v___x_11_;
}
else
{
return v___x_9_;
}
}
else
{
uint8_t v___x_12_; 
v___x_12_ = lean_uint64_dec_lt(v___x_10_, v_quantum_8_);
if (v___x_12_ == 0)
{
return v___x_12_;
}
else
{
return v___x_9_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_validConfig___boxed(lean_object* v_period_13_, lean_object* v_quantum_14_){
_start:
{
uint64_t v_period_boxed_15_; uint64_t v_quantum_boxed_16_; uint8_t v_res_17_; lean_object* v_r_18_; 
v_period_boxed_15_ = lean_unbox_uint64(v_period_13_);
lean_dec_ref(v_period_13_);
v_quantum_boxed_16_ = lean_unbox_uint64(v_quantum_14_);
lean_dec_ref(v_quantum_14_);
v_res_17_ = bastion_valid_config(v_period_boxed_15_, v_quantum_boxed_16_);
v_r_18_ = lean_box(v_res_17_);
return v_r_18_;
}
}
LEAN_EXPORT uint8_t bastion_valid_limits(uint64_t v_pageLimit_19_, uint64_t v_initialPages_20_, uint64_t v_capLimit_21_, uint64_t v_cpuBudget_22_, uint64_t v_period_23_){
_start:
{
uint8_t v___x_24_; uint8_t v___x_25_; uint8_t v___y_27_; uint64_t v___x_28_; uint8_t v___x_29_; 
v___x_24_ = lean_uint64_dec_le(v_initialPages_20_, v_pageLimit_19_);
v___x_25_ = lean_uint64_dec_le(v_cpuBudget_22_, v_period_23_);
v___x_28_ = 8ULL;
v___x_29_ = lean_uint64_dec_le(v_capLimit_21_, v___x_28_);
if (v___x_29_ == 0)
{
v___y_27_ = v___x_29_;
goto v___jp_26_;
}
else
{
uint64_t v___x_30_; uint8_t v___x_31_; 
v___x_30_ = 0ULL;
v___x_31_ = lean_uint64_dec_lt(v___x_30_, v_cpuBudget_22_);
v___y_27_ = v___x_31_;
goto v___jp_26_;
}
v___jp_26_:
{
if (v___y_27_ == 0)
{
return v___y_27_;
}
else
{
if (v___x_25_ == 0)
{
return v___x_25_;
}
else
{
return v___x_24_;
}
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_validLimits___boxed(lean_object* v_pageLimit_32_, lean_object* v_initialPages_33_, lean_object* v_capLimit_34_, lean_object* v_cpuBudget_35_, lean_object* v_period_36_){
_start:
{
uint64_t v_pageLimit_boxed_37_; uint64_t v_initialPages_boxed_38_; uint64_t v_capLimit_boxed_39_; uint64_t v_cpuBudget_boxed_40_; uint64_t v_period_boxed_41_; uint8_t v_res_42_; lean_object* v_r_43_; 
v_pageLimit_boxed_37_ = lean_unbox_uint64(v_pageLimit_32_);
lean_dec_ref(v_pageLimit_32_);
v_initialPages_boxed_38_ = lean_unbox_uint64(v_initialPages_33_);
lean_dec_ref(v_initialPages_33_);
v_capLimit_boxed_39_ = lean_unbox_uint64(v_capLimit_34_);
lean_dec_ref(v_capLimit_34_);
v_cpuBudget_boxed_40_ = lean_unbox_uint64(v_cpuBudget_35_);
lean_dec_ref(v_cpuBudget_35_);
v_period_boxed_41_ = lean_unbox_uint64(v_period_36_);
lean_dec_ref(v_period_36_);
v_res_42_ = bastion_valid_limits(v_pageLimit_boxed_37_, v_initialPages_boxed_38_, v_capLimit_boxed_39_, v_cpuBudget_boxed_40_, v_period_boxed_41_);
v_r_43_ = lean_box(v_res_42_);
return v_r_43_;
}
}
LEAN_EXPORT uint8_t bastion_can_reserve(uint64_t v_used_44_, uint64_t v_requested_45_, uint64_t v_limit_46_){
_start:
{
uint8_t v___x_47_; 
v___x_47_ = lean_uint64_dec_le(v_used_44_, v_limit_46_);
if (v___x_47_ == 0)
{
return v___x_47_;
}
else
{
uint64_t v___x_48_; uint8_t v___x_49_; 
v___x_48_ = lean_uint64_sub(v_limit_46_, v_used_44_);
v___x_49_ = lean_uint64_dec_le(v_requested_45_, v___x_48_);
return v___x_49_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_canReserve___boxed(lean_object* v_used_50_, lean_object* v_requested_51_, lean_object* v_limit_52_){
_start:
{
uint64_t v_used_boxed_53_; uint64_t v_requested_boxed_54_; uint64_t v_limit_boxed_55_; uint8_t v_res_56_; lean_object* v_r_57_; 
v_used_boxed_53_ = lean_unbox_uint64(v_used_50_);
lean_dec_ref(v_used_50_);
v_requested_boxed_54_ = lean_unbox_uint64(v_requested_51_);
lean_dec_ref(v_requested_51_);
v_limit_boxed_55_ = lean_unbox_uint64(v_limit_52_);
lean_dec_ref(v_limit_52_);
v_res_56_ = bastion_can_reserve(v_used_boxed_53_, v_requested_boxed_54_, v_limit_boxed_55_);
v_r_57_ = lean_box(v_res_56_);
return v_r_57_;
}
}
LEAN_EXPORT uint64_t bastion_reservation_status(uint64_t v_owned_58_, uint64_t v_total_59_, uint64_t v_requested_60_, uint64_t v_processLimit_61_, uint64_t v_globalLimit_62_){
_start:
{
uint8_t v___x_63_; 
v___x_63_ = bastion_can_reserve(v_owned_58_, v_requested_60_, v_processLimit_61_);
if (v___x_63_ == 0)
{
uint64_t v___x_64_; 
v___x_64_ = 1ULL;
return v___x_64_;
}
else
{
uint8_t v___x_65_; 
v___x_65_ = bastion_can_reserve(v_total_59_, v_requested_60_, v_globalLimit_62_);
if (v___x_65_ == 0)
{
uint64_t v___x_66_; 
v___x_66_ = 2ULL;
return v___x_66_;
}
else
{
uint64_t v___x_67_; 
v___x_67_ = 0ULL;
return v___x_67_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_reservationStatus___boxed(lean_object* v_owned_68_, lean_object* v_total_69_, lean_object* v_requested_70_, lean_object* v_processLimit_71_, lean_object* v_globalLimit_72_){
_start:
{
uint64_t v_owned_boxed_73_; uint64_t v_total_boxed_74_; uint64_t v_requested_boxed_75_; uint64_t v_processLimit_boxed_76_; uint64_t v_globalLimit_boxed_77_; uint64_t v_res_78_; lean_object* v_r_79_; 
v_owned_boxed_73_ = lean_unbox_uint64(v_owned_68_);
lean_dec_ref(v_owned_68_);
v_total_boxed_74_ = lean_unbox_uint64(v_total_69_);
lean_dec_ref(v_total_69_);
v_requested_boxed_75_ = lean_unbox_uint64(v_requested_70_);
lean_dec_ref(v_requested_70_);
v_processLimit_boxed_76_ = lean_unbox_uint64(v_processLimit_71_);
lean_dec_ref(v_processLimit_71_);
v_globalLimit_boxed_77_ = lean_unbox_uint64(v_globalLimit_72_);
lean_dec_ref(v_globalLimit_72_);
v_res_78_ = bastion_reservation_status(v_owned_boxed_73_, v_total_boxed_74_, v_requested_boxed_75_, v_processLimit_boxed_76_, v_globalLimit_boxed_77_);
v_r_79_ = lean_box_uint64(v_res_78_);
return v_r_79_;
}
}
LEAN_EXPORT uint64_t bastion_reserve_value(uint64_t v_used_80_, uint64_t v_requested_81_, uint64_t v_limit_82_){
_start:
{
uint8_t v___x_83_; 
v___x_83_ = bastion_can_reserve(v_used_80_, v_requested_81_, v_limit_82_);
if (v___x_83_ == 0)
{
return v_used_80_;
}
else
{
uint64_t v___x_84_; 
v___x_84_ = lean_uint64_add(v_used_80_, v_requested_81_);
return v___x_84_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_reserveValue___boxed(lean_object* v_used_85_, lean_object* v_requested_86_, lean_object* v_limit_87_){
_start:
{
uint64_t v_used_boxed_88_; uint64_t v_requested_boxed_89_; uint64_t v_limit_boxed_90_; uint64_t v_res_91_; lean_object* v_r_92_; 
v_used_boxed_88_ = lean_unbox_uint64(v_used_85_);
lean_dec_ref(v_used_85_);
v_requested_boxed_89_ = lean_unbox_uint64(v_requested_86_);
lean_dec_ref(v_requested_86_);
v_limit_boxed_90_ = lean_unbox_uint64(v_limit_87_);
lean_dec_ref(v_limit_87_);
v_res_91_ = bastion_reserve_value(v_used_boxed_88_, v_requested_boxed_89_, v_limit_boxed_90_);
v_r_92_ = lean_box_uint64(v_res_91_);
return v_r_92_;
}
}
LEAN_EXPORT uint8_t bastion_can_release(uint64_t v_used_93_, uint64_t v_amount_94_){
_start:
{
uint8_t v___x_95_; 
v___x_95_ = lean_uint64_dec_le(v_amount_94_, v_used_93_);
return v___x_95_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_canRelease___boxed(lean_object* v_used_96_, lean_object* v_amount_97_){
_start:
{
uint64_t v_used_boxed_98_; uint64_t v_amount_boxed_99_; uint8_t v_res_100_; lean_object* v_r_101_; 
v_used_boxed_98_ = lean_unbox_uint64(v_used_96_);
lean_dec_ref(v_used_96_);
v_amount_boxed_99_ = lean_unbox_uint64(v_amount_97_);
lean_dec_ref(v_amount_97_);
v_res_100_ = bastion_can_release(v_used_boxed_98_, v_amount_boxed_99_);
v_r_101_ = lean_box(v_res_100_);
return v_r_101_;
}
}
LEAN_EXPORT uint64_t bastion_release_value(uint64_t v_used_102_, uint64_t v_amount_103_){
_start:
{
uint8_t v___x_104_; 
v___x_104_ = lean_uint64_dec_le(v_amount_103_, v_used_102_);
if (v___x_104_ == 0)
{
return v_used_102_;
}
else
{
uint64_t v___x_105_; 
v___x_105_ = lean_uint64_sub(v_used_102_, v_amount_103_);
return v___x_105_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_releaseValue___boxed(lean_object* v_used_106_, lean_object* v_amount_107_){
_start:
{
uint64_t v_used_boxed_108_; uint64_t v_amount_boxed_109_; uint64_t v_res_110_; lean_object* v_r_111_; 
v_used_boxed_108_ = lean_unbox_uint64(v_used_106_);
lean_dec_ref(v_used_106_);
v_amount_boxed_109_ = lean_unbox_uint64(v_amount_107_);
lean_dec_ref(v_amount_107_);
v_res_110_ = bastion_release_value(v_used_boxed_108_, v_amount_boxed_109_);
v_r_111_ = lean_box_uint64(v_res_110_);
return v_r_111_;
}
}
LEAN_EXPORT uint64_t bastion_next_identifier(uint64_t v_current_112_){
_start:
{
uint64_t v___x_113_; uint8_t v___y_115_; uint8_t v___x_118_; 
v___x_113_ = 0ULL;
v___x_118_ = lean_uint64_dec_eq(v_current_112_, v___x_113_);
if (v___x_118_ == 0)
{
uint64_t v___x_119_; uint8_t v___x_120_; 
v___x_119_ = 18446744073709551615ULL;
v___x_120_ = lean_uint64_dec_eq(v_current_112_, v___x_119_);
v___y_115_ = v___x_120_;
goto v___jp_114_;
}
else
{
v___y_115_ = v___x_118_;
goto v___jp_114_;
}
v___jp_114_:
{
if (v___y_115_ == 0)
{
uint64_t v___x_116_; uint64_t v___x_117_; 
v___x_116_ = 1ULL;
v___x_117_ = lean_uint64_add(v_current_112_, v___x_116_);
return v___x_117_;
}
else
{
return v___x_113_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextIdentifier___boxed(lean_object* v_current_121_){
_start:
{
uint64_t v_current_boxed_122_; uint64_t v_res_123_; lean_object* v_r_124_; 
v_current_boxed_122_ = lean_unbox_uint64(v_current_121_);
lean_dec_ref(v_current_121_);
v_res_123_ = bastion_next_identifier(v_current_boxed_122_);
v_r_124_ = lean_box_uint64(v_res_123_);
return v_r_124_;
}
}
LEAN_EXPORT uint8_t bastion_same_identity(uint64_t v_stored_125_, uint64_t v_requested_126_){
_start:
{
uint64_t v___x_127_; uint8_t v___x_128_; 
v___x_127_ = 0ULL;
v___x_128_ = lean_uint64_dec_eq(v_stored_125_, v___x_127_);
if (v___x_128_ == 0)
{
uint8_t v___x_129_; 
v___x_129_ = lean_uint64_dec_eq(v_stored_125_, v_requested_126_);
return v___x_129_;
}
else
{
uint8_t v___x_130_; 
v___x_130_ = 0;
return v___x_130_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_sameIdentity___boxed(lean_object* v_stored_131_, lean_object* v_requested_132_){
_start:
{
uint64_t v_stored_boxed_133_; uint64_t v_requested_boxed_134_; uint8_t v_res_135_; lean_object* v_r_136_; 
v_stored_boxed_133_ = lean_unbox_uint64(v_stored_131_);
lean_dec_ref(v_stored_131_);
v_requested_boxed_134_ = lean_unbox_uint64(v_requested_132_);
lean_dec_ref(v_requested_132_);
v_res_135_ = bastion_same_identity(v_stored_boxed_133_, v_requested_boxed_134_);
v_r_136_ = lean_box(v_res_135_);
return v_r_136_;
}
}
LEAN_EXPORT uint8_t bastion_can_grant(uint64_t v_count_137_, uint64_t v_limit_138_){
_start:
{
uint8_t v___x_139_; 
v___x_139_ = lean_uint64_dec_lt(v_count_137_, v_limit_138_);
if (v___x_139_ == 0)
{
return v___x_139_;
}
else
{
uint64_t v___x_140_; uint8_t v___x_141_; 
v___x_140_ = 8ULL;
v___x_141_ = lean_uint64_dec_le(v_limit_138_, v___x_140_);
return v___x_141_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_canGrant___boxed(lean_object* v_count_142_, lean_object* v_limit_143_){
_start:
{
uint64_t v_count_boxed_144_; uint64_t v_limit_boxed_145_; uint8_t v_res_146_; lean_object* v_r_147_; 
v_count_boxed_144_ = lean_unbox_uint64(v_count_142_);
lean_dec_ref(v_count_142_);
v_limit_boxed_145_ = lean_unbox_uint64(v_limit_143_);
lean_dec_ref(v_limit_143_);
v_res_146_ = bastion_can_grant(v_count_boxed_144_, v_limit_boxed_145_);
v_r_147_ = lean_box(v_res_146_);
return v_r_147_;
}
}
LEAN_EXPORT uint8_t bastion_rights_allow(uint64_t v_held_148_, uint64_t v_needed_149_){
_start:
{
uint8_t v___y_151_; uint64_t v___x_154_; uint8_t v___x_155_; 
v___x_154_ = 3ULL;
v___x_155_ = lean_uint64_dec_le(v_held_148_, v___x_154_);
if (v___x_155_ == 0)
{
v___y_151_ = v___x_155_;
goto v___jp_150_;
}
else
{
uint8_t v___x_156_; 
v___x_156_ = lean_uint64_dec_le(v_needed_149_, v___x_154_);
v___y_151_ = v___x_156_;
goto v___jp_150_;
}
v___jp_150_:
{
if (v___y_151_ == 0)
{
return v___y_151_;
}
else
{
uint64_t v___x_152_; uint8_t v___x_153_; 
v___x_152_ = lean_uint64_land(v_held_148_, v_needed_149_);
v___x_153_ = lean_uint64_dec_eq(v___x_152_, v_needed_149_);
return v___x_153_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_rightsAllow___boxed(lean_object* v_held_157_, lean_object* v_needed_158_){
_start:
{
uint64_t v_held_boxed_159_; uint64_t v_needed_boxed_160_; uint8_t v_res_161_; lean_object* v_r_162_; 
v_held_boxed_159_ = lean_unbox_uint64(v_held_157_);
lean_dec_ref(v_held_157_);
v_needed_boxed_160_ = lean_unbox_uint64(v_needed_158_);
lean_dec_ref(v_needed_158_);
v_res_161_ = bastion_rights_allow(v_held_boxed_159_, v_needed_boxed_160_);
v_r_162_ = lean_box(v_res_161_);
return v_r_162_;
}
}
LEAN_EXPORT uint8_t bastion_authorized(uint64_t v_caller_163_, uint64_t v_owner_164_, uint64_t v_targetLive_165_, uint64_t v_held_166_, uint64_t v_needed_167_){
_start:
{
uint8_t v___y_169_; uint8_t v___x_171_; 
v___x_171_ = bastion_same_identity(v_caller_163_, v_owner_164_);
if (v___x_171_ == 0)
{
v___y_169_ = v___x_171_;
goto v___jp_168_;
}
else
{
uint64_t v___x_172_; uint8_t v___x_173_; 
v___x_172_ = 1ULL;
v___x_173_ = lean_uint64_dec_eq(v_targetLive_165_, v___x_172_);
v___y_169_ = v___x_173_;
goto v___jp_168_;
}
v___jp_168_:
{
if (v___y_169_ == 0)
{
return v___y_169_;
}
else
{
uint8_t v___x_170_; 
v___x_170_ = bastion_rights_allow(v_held_166_, v_needed_167_);
return v___x_170_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_authorized___boxed(lean_object* v_caller_174_, lean_object* v_owner_175_, lean_object* v_targetLive_176_, lean_object* v_held_177_, lean_object* v_needed_178_){
_start:
{
uint64_t v_caller_boxed_179_; uint64_t v_owner_boxed_180_; uint64_t v_targetLive_boxed_181_; uint64_t v_held_boxed_182_; uint64_t v_needed_boxed_183_; uint8_t v_res_184_; lean_object* v_r_185_; 
v_caller_boxed_179_ = lean_unbox_uint64(v_caller_174_);
lean_dec_ref(v_caller_174_);
v_owner_boxed_180_ = lean_unbox_uint64(v_owner_175_);
lean_dec_ref(v_owner_175_);
v_targetLive_boxed_181_ = lean_unbox_uint64(v_targetLive_176_);
lean_dec_ref(v_targetLive_176_);
v_held_boxed_182_ = lean_unbox_uint64(v_held_177_);
lean_dec_ref(v_held_177_);
v_needed_boxed_183_ = lean_unbox_uint64(v_needed_178_);
lean_dec_ref(v_needed_178_);
v_res_184_ = bastion_authorized(v_caller_boxed_179_, v_owner_boxed_180_, v_targetLive_boxed_181_, v_held_boxed_182_, v_needed_boxed_183_);
v_r_185_ = lean_box(v_res_184_);
return v_r_185_;
}
}
LEAN_EXPORT uint64_t bastion_restrict_rights(uint64_t v_held_186_, uint64_t v_requested_187_){
_start:
{
uint8_t v___x_188_; 
v___x_188_ = bastion_rights_allow(v_held_186_, v_requested_187_);
if (v___x_188_ == 0)
{
uint64_t v___x_189_; 
v___x_189_ = 4ULL;
return v___x_189_;
}
else
{
return v_requested_187_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_restrictRights___boxed(lean_object* v_held_190_, lean_object* v_requested_191_){
_start:
{
uint64_t v_held_boxed_192_; uint64_t v_requested_boxed_193_; uint64_t v_res_194_; lean_object* v_r_195_; 
v_held_boxed_192_ = lean_unbox_uint64(v_held_190_);
lean_dec_ref(v_held_190_);
v_requested_boxed_193_ = lean_unbox_uint64(v_requested_191_);
lean_dec_ref(v_requested_191_);
v_res_194_ = bastion_restrict_rights(v_held_boxed_192_, v_requested_boxed_193_);
v_r_195_ = lean_box_uint64(v_res_194_);
return v_r_195_;
}
}
LEAN_EXPORT uint8_t bastion_revoke_target(uint64_t v_capTarget_196_, uint64_t v_removed_197_){
_start:
{
uint8_t v___x_198_; 
v___x_198_ = bastion_same_identity(v_capTarget_196_, v_removed_197_);
return v___x_198_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_revokeTarget___boxed(lean_object* v_capTarget_199_, lean_object* v_removed_200_){
_start:
{
uint64_t v_capTarget_boxed_201_; uint64_t v_removed_boxed_202_; uint8_t v_res_203_; lean_object* v_r_204_; 
v_capTarget_boxed_201_ = lean_unbox_uint64(v_capTarget_199_);
lean_dec_ref(v_capTarget_199_);
v_removed_boxed_202_ = lean_unbox_uint64(v_removed_200_);
lean_dec_ref(v_removed_200_);
v_res_203_ = bastion_revoke_target(v_capTarget_boxed_201_, v_removed_boxed_202_);
v_r_204_ = lean_box(v_res_203_);
return v_r_204_;
}
}
LEAN_EXPORT uint64_t bastion_charge(uint64_t v_remaining_205_, uint64_t v_elapsed_206_){
_start:
{
uint8_t v___x_207_; 
v___x_207_ = lean_uint64_dec_le(v_elapsed_206_, v_remaining_205_);
if (v___x_207_ == 0)
{
uint64_t v___x_208_; 
v___x_208_ = 0ULL;
return v___x_208_;
}
else
{
uint64_t v___x_209_; 
v___x_209_ = lean_uint64_sub(v_remaining_205_, v_elapsed_206_);
return v___x_209_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_charge___boxed(lean_object* v_remaining_210_, lean_object* v_elapsed_211_){
_start:
{
uint64_t v_remaining_boxed_212_; uint64_t v_elapsed_boxed_213_; uint64_t v_res_214_; lean_object* v_r_215_; 
v_remaining_boxed_212_ = lean_unbox_uint64(v_remaining_210_);
lean_dec_ref(v_remaining_210_);
v_elapsed_boxed_213_ = lean_unbox_uint64(v_elapsed_211_);
lean_dec_ref(v_elapsed_211_);
v_res_214_ = bastion_charge(v_remaining_boxed_212_, v_elapsed_boxed_213_);
v_r_215_ = lean_box_uint64(v_res_214_);
return v_r_215_;
}
}
LEAN_EXPORT uint8_t bastion_clock_valid(uint64_t v_old_216_, uint64_t v_now_217_){
_start:
{
uint8_t v___x_218_; 
v___x_218_ = lean_uint64_dec_le(v_old_216_, v_now_217_);
return v___x_218_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_clockValid___boxed(lean_object* v_old_219_, lean_object* v_now_220_){
_start:
{
uint64_t v_old_boxed_221_; uint64_t v_now_boxed_222_; uint8_t v_res_223_; lean_object* v_r_224_; 
v_old_boxed_221_ = lean_unbox_uint64(v_old_219_);
lean_dec_ref(v_old_219_);
v_now_boxed_222_ = lean_unbox_uint64(v_now_220_);
lean_dec_ref(v_now_220_);
v_res_223_ = bastion_clock_valid(v_old_boxed_221_, v_now_boxed_222_);
v_r_224_ = lean_box(v_res_223_);
return v_r_224_;
}
}
LEAN_EXPORT uint64_t bastion_account(uint64_t v_remaining_225_, uint64_t v_budget_226_, uint64_t v_old_227_, uint64_t v_now_228_, uint64_t v_period_229_, uint64_t v_active_230_){
_start:
{
uint64_t v___y_232_; uint64_t v___x_244_; uint8_t v___y_246_; uint8_t v___x_248_; 
v___x_244_ = 0ULL;
v___x_248_ = lean_uint64_dec_eq(v_period_229_, v___x_244_);
if (v___x_248_ == 0)
{
uint8_t v___x_249_; 
v___x_249_ = lean_uint64_dec_le(v_old_227_, v_now_228_);
if (v___x_249_ == 0)
{
return v___x_244_;
}
else
{
v___y_246_ = v___x_248_;
goto v___jp_245_;
}
}
else
{
v___y_246_ = v___x_248_;
goto v___jp_245_;
}
v___jp_231_:
{
uint64_t v___x_233_; uint64_t v___x_234_; uint8_t v___x_235_; 
v___x_233_ = lean_uint64_div(v_old_227_, v_period_229_);
v___x_234_ = lean_uint64_div(v_now_228_, v_period_229_);
v___x_235_ = lean_uint64_dec_eq(v___x_233_, v___x_234_);
if (v___x_235_ == 0)
{
uint64_t v___x_236_; uint8_t v___x_237_; 
v___x_236_ = 1ULL;
v___x_237_ = lean_uint64_dec_eq(v_active_230_, v___x_236_);
if (v___x_237_ == 0)
{
return v_budget_226_;
}
else
{
uint64_t v___x_238_; uint64_t v___x_239_; 
v___x_238_ = lean_uint64_mod(v_now_228_, v_period_229_);
v___x_239_ = bastion_charge(v_budget_226_, v___x_238_);
return v___x_239_;
}
}
else
{
uint64_t v___x_240_; uint8_t v___x_241_; 
v___x_240_ = 1ULL;
v___x_241_ = lean_uint64_dec_eq(v_active_230_, v___x_240_);
if (v___x_241_ == 0)
{
return v___y_232_;
}
else
{
uint64_t v___x_242_; uint64_t v___x_243_; 
v___x_242_ = lean_uint64_sub(v_now_228_, v_old_227_);
v___x_243_ = bastion_charge(v___y_232_, v___x_242_);
return v___x_243_;
}
}
}
v___jp_245_:
{
if (v___y_246_ == 0)
{
uint8_t v___x_247_; 
v___x_247_ = lean_uint64_dec_le(v_remaining_225_, v_budget_226_);
if (v___x_247_ == 0)
{
v___y_232_ = v_budget_226_;
goto v___jp_231_;
}
else
{
v___y_232_ = v_remaining_225_;
goto v___jp_231_;
}
}
else
{
return v___x_244_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_account___boxed(lean_object* v_remaining_250_, lean_object* v_budget_251_, lean_object* v_old_252_, lean_object* v_now_253_, lean_object* v_period_254_, lean_object* v_active_255_){
_start:
{
uint64_t v_remaining_boxed_256_; uint64_t v_budget_boxed_257_; uint64_t v_old_boxed_258_; uint64_t v_now_boxed_259_; uint64_t v_period_boxed_260_; uint64_t v_active_boxed_261_; uint64_t v_res_262_; lean_object* v_r_263_; 
v_remaining_boxed_256_ = lean_unbox_uint64(v_remaining_250_);
lean_dec_ref(v_remaining_250_);
v_budget_boxed_257_ = lean_unbox_uint64(v_budget_251_);
lean_dec_ref(v_budget_251_);
v_old_boxed_258_ = lean_unbox_uint64(v_old_252_);
lean_dec_ref(v_old_252_);
v_now_boxed_259_ = lean_unbox_uint64(v_now_253_);
lean_dec_ref(v_now_253_);
v_period_boxed_260_ = lean_unbox_uint64(v_period_254_);
lean_dec_ref(v_period_254_);
v_active_boxed_261_ = lean_unbox_uint64(v_active_255_);
lean_dec_ref(v_active_255_);
v_res_262_ = bastion_account(v_remaining_boxed_256_, v_budget_boxed_257_, v_old_boxed_258_, v_now_boxed_259_, v_period_boxed_260_, v_active_boxed_261_);
v_r_263_ = lean_box_uint64(v_res_262_);
return v_r_263_;
}
}
LEAN_EXPORT uint8_t bastion_runnable(uint64_t v_alive_264_, uint64_t v_remaining_265_){
_start:
{
uint64_t v___x_266_; uint8_t v___x_267_; 
v___x_266_ = 1ULL;
v___x_267_ = lean_uint64_dec_eq(v_alive_264_, v___x_266_);
if (v___x_267_ == 0)
{
return v___x_267_;
}
else
{
uint64_t v___x_268_; uint8_t v___x_269_; 
v___x_268_ = 0ULL;
v___x_269_ = lean_uint64_dec_lt(v___x_268_, v_remaining_265_);
return v___x_269_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_runnable___boxed(lean_object* v_alive_270_, lean_object* v_remaining_271_){
_start:
{
uint64_t v_alive_boxed_272_; uint64_t v_remaining_boxed_273_; uint8_t v_res_274_; lean_object* v_r_275_; 
v_alive_boxed_272_ = lean_unbox_uint64(v_alive_270_);
lean_dec_ref(v_alive_270_);
v_remaining_boxed_273_ = lean_unbox_uint64(v_remaining_271_);
lean_dec_ref(v_remaining_271_);
v_res_274_ = bastion_runnable(v_alive_boxed_272_, v_remaining_boxed_273_);
v_r_275_ = lean_box(v_res_274_);
return v_r_275_;
}
}
LEAN_EXPORT uint8_t l_Bastion_Runtime_eligible(uint64_t v_mask_276_, uint64_t v_index_277_){
_start:
{
uint64_t v___x_278_; uint64_t v___x_279_; uint64_t v___x_280_; uint64_t v___x_281_; uint8_t v___x_282_; 
v___x_278_ = 1ULL;
v___x_279_ = lean_uint64_shift_left(v___x_278_, v_index_277_);
v___x_280_ = lean_uint64_land(v_mask_276_, v___x_279_);
v___x_281_ = 0ULL;
v___x_282_ = lean_uint64_dec_eq(v___x_280_, v___x_281_);
if (v___x_282_ == 0)
{
uint8_t v___x_283_; 
v___x_283_ = 1;
return v___x_283_;
}
else
{
uint8_t v___x_284_; 
v___x_284_ = 0;
return v___x_284_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_eligible___boxed(lean_object* v_mask_285_, lean_object* v_index_286_){
_start:
{
uint64_t v_mask_boxed_287_; uint64_t v_index_boxed_288_; uint8_t v_res_289_; lean_object* v_r_290_; 
v_mask_boxed_287_ = lean_unbox_uint64(v_mask_285_);
lean_dec_ref(v_mask_285_);
v_index_boxed_288_ = lean_unbox_uint64(v_index_286_);
lean_dec_ref(v_index_286_);
v_res_289_ = l_Bastion_Runtime_eligible(v_mask_boxed_287_, v_index_boxed_288_);
v_r_290_ = lean_box(v_res_289_);
return v_r_290_;
}
}
LEAN_EXPORT uint64_t bastion_next_slot(uint64_t v_mask_291_, uint64_t v_cursor_292_){
_start:
{
uint64_t v___x_293_; uint8_t v___x_294_; 
v___x_293_ = 8ULL;
v___x_294_ = lean_uint64_dec_le(v___x_293_, v_cursor_292_);
if (v___x_294_ == 0)
{
uint64_t v___x_295_; uint64_t v___x_296_; uint64_t v_i1_297_; uint64_t v___x_298_; uint64_t v___x_299_; uint64_t v_i2_300_; uint64_t v___x_301_; uint64_t v___x_302_; uint64_t v_i3_303_; uint64_t v___x_304_; uint64_t v___x_305_; uint64_t v_i4_306_; uint64_t v___x_307_; uint64_t v___x_308_; uint64_t v___x_309_; uint64_t v___x_310_; uint64_t v___x_311_; uint64_t v___x_312_; uint64_t v___x_313_; uint64_t v___x_314_; uint64_t v___x_315_; uint8_t v___x_316_; 
v___x_295_ = 1ULL;
v___x_296_ = lean_uint64_add(v_cursor_292_, v___x_295_);
v_i1_297_ = lean_uint64_mod(v___x_296_, v___x_293_);
v___x_298_ = 2ULL;
v___x_299_ = lean_uint64_add(v_cursor_292_, v___x_298_);
v_i2_300_ = lean_uint64_mod(v___x_299_, v___x_293_);
v___x_301_ = 3ULL;
v___x_302_ = lean_uint64_add(v_cursor_292_, v___x_301_);
v_i3_303_ = lean_uint64_mod(v___x_302_, v___x_293_);
v___x_304_ = 4ULL;
v___x_305_ = lean_uint64_add(v_cursor_292_, v___x_304_);
v_i4_306_ = lean_uint64_mod(v___x_305_, v___x_293_);
v___x_307_ = 5ULL;
v___x_308_ = lean_uint64_add(v_cursor_292_, v___x_307_);
v___x_309_ = 6ULL;
v___x_310_ = lean_uint64_add(v_cursor_292_, v___x_309_);
v___x_311_ = 7ULL;
v___x_312_ = lean_uint64_add(v_cursor_292_, v___x_311_);
v___x_313_ = lean_uint64_shift_left(v___x_295_, v_cursor_292_);
v___x_314_ = lean_uint64_land(v_mask_291_, v___x_313_);
v___x_315_ = 0ULL;
v___x_316_ = lean_uint64_dec_eq(v___x_314_, v___x_315_);
if (v___x_316_ == 0)
{
return v_cursor_292_;
}
else
{
uint64_t v___x_317_; uint64_t v___x_318_; uint8_t v___x_319_; 
v___x_317_ = lean_uint64_shift_left(v___x_295_, v_i1_297_);
v___x_318_ = lean_uint64_land(v_mask_291_, v___x_317_);
v___x_319_ = lean_uint64_dec_eq(v___x_318_, v___x_315_);
if (v___x_319_ == 0)
{
return v_i1_297_;
}
else
{
uint64_t v___x_320_; uint64_t v___x_321_; uint8_t v___x_322_; 
v___x_320_ = lean_uint64_shift_left(v___x_295_, v_i2_300_);
v___x_321_ = lean_uint64_land(v_mask_291_, v___x_320_);
v___x_322_ = lean_uint64_dec_eq(v___x_321_, v___x_315_);
if (v___x_322_ == 0)
{
return v_i2_300_;
}
else
{
uint64_t v___x_323_; uint64_t v___x_324_; uint8_t v___x_325_; 
v___x_323_ = lean_uint64_shift_left(v___x_295_, v_i3_303_);
v___x_324_ = lean_uint64_land(v_mask_291_, v___x_323_);
v___x_325_ = lean_uint64_dec_eq(v___x_324_, v___x_315_);
if (v___x_325_ == 0)
{
return v_i3_303_;
}
else
{
uint64_t v___x_326_; uint64_t v___x_327_; uint8_t v___x_328_; 
v___x_326_ = lean_uint64_shift_left(v___x_295_, v_i4_306_);
v___x_327_ = lean_uint64_land(v_mask_291_, v___x_326_);
v___x_328_ = lean_uint64_dec_eq(v___x_327_, v___x_315_);
if (v___x_328_ == 0)
{
return v_i4_306_;
}
else
{
uint64_t v_i5_329_; uint64_t v___x_330_; uint64_t v___x_331_; uint8_t v___x_332_; 
v_i5_329_ = lean_uint64_mod(v___x_308_, v___x_293_);
v___x_330_ = lean_uint64_shift_left(v___x_295_, v_i5_329_);
v___x_331_ = lean_uint64_land(v_mask_291_, v___x_330_);
v___x_332_ = lean_uint64_dec_eq(v___x_331_, v___x_315_);
if (v___x_332_ == 0)
{
return v_i5_329_;
}
else
{
uint64_t v_i6_333_; uint64_t v___x_334_; uint64_t v___x_335_; uint8_t v___x_336_; 
v_i6_333_ = lean_uint64_mod(v___x_310_, v___x_293_);
v___x_334_ = lean_uint64_shift_left(v___x_295_, v_i6_333_);
v___x_335_ = lean_uint64_land(v_mask_291_, v___x_334_);
v___x_336_ = lean_uint64_dec_eq(v___x_335_, v___x_315_);
if (v___x_336_ == 0)
{
return v_i6_333_;
}
else
{
uint64_t v_i7_337_; uint64_t v___x_338_; uint64_t v___x_339_; uint8_t v___x_340_; 
v_i7_337_ = lean_uint64_mod(v___x_312_, v___x_293_);
v___x_338_ = lean_uint64_shift_left(v___x_295_, v_i7_337_);
v___x_339_ = lean_uint64_land(v_mask_291_, v___x_338_);
v___x_340_ = lean_uint64_dec_eq(v___x_339_, v___x_315_);
if (v___x_340_ == 0)
{
return v_i7_337_;
}
else
{
return v___x_293_;
}
}
}
}
}
}
}
}
}
else
{
return v___x_293_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextSlot___boxed(lean_object* v_mask_341_, lean_object* v_cursor_342_){
_start:
{
uint64_t v_mask_boxed_343_; uint64_t v_cursor_boxed_344_; uint64_t v_res_345_; lean_object* v_r_346_; 
v_mask_boxed_343_ = lean_unbox_uint64(v_mask_341_);
lean_dec_ref(v_mask_341_);
v_cursor_boxed_344_ = lean_unbox_uint64(v_cursor_342_);
lean_dec_ref(v_cursor_342_);
v_res_345_ = bastion_next_slot(v_mask_boxed_343_, v_cursor_boxed_344_);
v_r_346_ = lean_box_uint64(v_res_345_);
return v_r_346_;
}
}
LEAN_EXPORT uint64_t bastion_next_cursor(uint64_t v_selected_347_){
_start:
{
uint64_t v___x_348_; uint8_t v___x_349_; 
v___x_348_ = 8ULL;
v___x_349_ = lean_uint64_dec_lt(v_selected_347_, v___x_348_);
if (v___x_349_ == 0)
{
uint64_t v___x_350_; 
v___x_350_ = 0ULL;
return v___x_350_;
}
else
{
uint64_t v___x_351_; uint64_t v___x_352_; uint64_t v___x_353_; 
v___x_351_ = 1ULL;
v___x_352_ = lean_uint64_add(v_selected_347_, v___x_351_);
v___x_353_ = lean_uint64_mod(v___x_352_, v___x_348_);
return v___x_353_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_nextCursor___boxed(lean_object* v_selected_354_){
_start:
{
uint64_t v_selected_boxed_355_; uint64_t v_res_356_; lean_object* v_r_357_; 
v_selected_boxed_355_ = lean_unbox_uint64(v_selected_354_);
lean_dec_ref(v_selected_354_);
v_res_356_ = bastion_next_cursor(v_selected_boxed_355_);
v_r_357_ = lean_box_uint64(v_res_356_);
return v_r_357_;
}
}
LEAN_EXPORT uint64_t bastion_deadline(uint64_t v_now_358_, uint64_t v_period_359_, uint64_t v_quantum_360_, uint64_t v_remaining_361_, uint64_t v_active_362_){
_start:
{
uint64_t v___y_364_; uint64_t v___y_370_; uint64_t v___x_372_; uint8_t v___x_373_; 
v___x_372_ = 0ULL;
v___x_373_ = lean_uint64_dec_eq(v_period_359_, v___x_372_);
if (v___x_373_ == 0)
{
uint64_t v___x_374_; uint64_t v_toPeriod_375_; uint64_t v___x_376_; uint8_t v___x_377_; 
v___x_374_ = lean_uint64_mod(v_now_358_, v_period_359_);
v_toPeriod_375_ = lean_uint64_sub(v_period_359_, v___x_374_);
v___x_376_ = 1ULL;
v___x_377_ = lean_uint64_dec_eq(v_active_362_, v___x_376_);
if (v___x_377_ == 0)
{
v___y_364_ = v_toPeriod_375_;
goto v___jp_363_;
}
else
{
uint8_t v___x_378_; 
v___x_378_ = lean_uint64_dec_le(v_toPeriod_375_, v_remaining_361_);
if (v___x_378_ == 0)
{
v___y_370_ = v_remaining_361_;
goto v___jp_369_;
}
else
{
v___y_370_ = v_toPeriod_375_;
goto v___jp_369_;
}
}
}
else
{
return v_now_358_;
}
v___jp_363_:
{
uint64_t v___x_365_; uint64_t v___x_366_; uint8_t v___x_367_; 
v___x_365_ = 18446744073709551615ULL;
v___x_366_ = lean_uint64_sub(v___x_365_, v_now_358_);
v___x_367_ = lean_uint64_dec_le(v___y_364_, v___x_366_);
if (v___x_367_ == 0)
{
return v___x_365_;
}
else
{
uint64_t v___x_368_; 
v___x_368_ = lean_uint64_add(v_now_358_, v___y_364_);
return v___x_368_;
}
}
v___jp_369_:
{
uint8_t v___x_371_; 
v___x_371_ = lean_uint64_dec_le(v_quantum_360_, v___y_370_);
if (v___x_371_ == 0)
{
v___y_364_ = v___y_370_;
goto v___jp_363_;
}
else
{
v___y_364_ = v_quantum_360_;
goto v___jp_363_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_deadline___boxed(lean_object* v_now_379_, lean_object* v_period_380_, lean_object* v_quantum_381_, lean_object* v_remaining_382_, lean_object* v_active_383_){
_start:
{
uint64_t v_now_boxed_384_; uint64_t v_period_boxed_385_; uint64_t v_quantum_boxed_386_; uint64_t v_remaining_boxed_387_; uint64_t v_active_boxed_388_; uint64_t v_res_389_; lean_object* v_r_390_; 
v_now_boxed_384_ = lean_unbox_uint64(v_now_379_);
lean_dec_ref(v_now_379_);
v_period_boxed_385_ = lean_unbox_uint64(v_period_380_);
lean_dec_ref(v_period_380_);
v_quantum_boxed_386_ = lean_unbox_uint64(v_quantum_381_);
lean_dec_ref(v_quantum_381_);
v_remaining_boxed_387_ = lean_unbox_uint64(v_remaining_382_);
lean_dec_ref(v_remaining_382_);
v_active_boxed_388_ = lean_unbox_uint64(v_active_383_);
lean_dec_ref(v_active_383_);
v_res_389_ = bastion_deadline(v_now_boxed_384_, v_period_boxed_385_, v_quantum_boxed_386_, v_remaining_boxed_387_, v_active_boxed_388_);
v_r_390_ = lean_box_uint64(v_res_389_);
return v_r_390_;
}
}
LEAN_EXPORT uint8_t bastion_valid_user_return(uint64_t v_rip_391_, uint64_t v_rsp_392_){
_start:
{
uint64_t v___x_393_; uint8_t v___x_394_; 
v___x_393_ = 140737488355328ULL;
v___x_394_ = lean_uint64_dec_lt(v_rip_391_, v___x_393_);
if (v___x_394_ == 0)
{
return v___x_394_;
}
else
{
uint8_t v___x_395_; 
v___x_395_ = lean_uint64_dec_lt(v_rsp_392_, v___x_393_);
return v___x_395_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_validUserReturn___boxed(lean_object* v_rip_396_, lean_object* v_rsp_397_){
_start:
{
uint64_t v_rip_boxed_398_; uint64_t v_rsp_boxed_399_; uint8_t v_res_400_; lean_object* v_r_401_; 
v_rip_boxed_398_ = lean_unbox_uint64(v_rip_396_);
lean_dec_ref(v_rip_396_);
v_rsp_boxed_399_ = lean_unbox_uint64(v_rsp_397_);
lean_dec_ref(v_rsp_397_);
v_res_400_ = bastion_valid_user_return(v_rip_boxed_398_, v_rsp_boxed_399_);
v_r_401_ = lean_box(v_res_400_);
return v_r_401_;
}
}
LEAN_EXPORT uint64_t bastion_user_flags(uint64_t v_flags_402_){
_start:
{
uint64_t v___x_403_; uint64_t v___x_404_; uint64_t v___x_405_; uint64_t v___x_406_; 
v___x_403_ = 3285ULL;
v___x_404_ = lean_uint64_land(v_flags_402_, v___x_403_);
v___x_405_ = 514ULL;
v___x_406_ = lean_uint64_lor(v___x_404_, v___x_405_);
return v___x_406_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_userFlags___boxed(lean_object* v_flags_407_){
_start:
{
uint64_t v_flags_boxed_408_; uint64_t v_res_409_; lean_object* v_r_410_; 
v_flags_boxed_408_ = lean_unbox_uint64(v_flags_407_);
lean_dec_ref(v_flags_407_);
v_res_409_ = bastion_user_flags(v_flags_boxed_408_);
v_r_410_ = lean_box_uint64(v_res_409_);
return v_r_410_;
}
}
LEAN_EXPORT uint64_t bastion_supervisor_entry(uint64_t v_entry_411_){
_start:
{
uint64_t v___x_412_; uint64_t v___x_413_; 
v___x_412_ = 18446744073709551611ULL;
v___x_413_ = lean_uint64_land(v_entry_411_, v___x_412_);
return v___x_413_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_supervisorEntry___boxed(lean_object* v_entry_414_){
_start:
{
uint64_t v_entry_boxed_415_; uint64_t v_res_416_; lean_object* v_r_417_; 
v_entry_boxed_415_ = lean_unbox_uint64(v_entry_414_);
lean_dec_ref(v_entry_414_);
v_res_416_ = bastion_supervisor_entry(v_entry_boxed_415_);
v_r_417_ = lean_box_uint64(v_res_416_);
return v_r_417_;
}
}
LEAN_EXPORT uint64_t bastion_user_page_entry(uint64_t v_physical_418_, uint64_t v_kind_419_){
_start:
{
uint64_t v___x_420_; uint64_t v___x_421_; uint64_t v___x_422_; uint8_t v___x_423_; 
v___x_420_ = 4095ULL;
v___x_421_ = lean_uint64_land(v_physical_418_, v___x_420_);
v___x_422_ = 0ULL;
v___x_423_ = lean_uint64_dec_eq(v___x_421_, v___x_422_);
if (v___x_423_ == 0)
{
return v___x_422_;
}
else
{
uint64_t v___x_424_; uint8_t v___x_425_; 
v___x_424_ = 4503599627370496ULL;
v___x_425_ = lean_uint64_dec_le(v___x_424_, v_physical_418_);
if (v___x_425_ == 0)
{
uint64_t v___x_426_; uint8_t v___x_427_; 
v___x_426_ = 1ULL;
v___x_427_ = lean_uint64_dec_eq(v_kind_419_, v___x_426_);
if (v___x_427_ == 0)
{
uint64_t v___x_428_; uint8_t v___x_429_; 
v___x_428_ = 2ULL;
v___x_429_ = lean_uint64_dec_eq(v_kind_419_, v___x_428_);
if (v___x_429_ == 0)
{
return v___x_422_;
}
else
{
uint64_t v___x_430_; uint64_t v___x_431_; 
v___x_430_ = 9223372036854775815ULL;
v___x_431_ = lean_uint64_lor(v_physical_418_, v___x_430_);
return v___x_431_;
}
}
else
{
uint64_t v___x_432_; uint64_t v___x_433_; 
v___x_432_ = 5ULL;
v___x_433_ = lean_uint64_lor(v_physical_418_, v___x_432_);
return v___x_433_;
}
}
else
{
return v___x_422_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_userPageEntry___boxed(lean_object* v_physical_434_, lean_object* v_kind_435_){
_start:
{
uint64_t v_physical_boxed_436_; uint64_t v_kind_boxed_437_; uint64_t v_res_438_; lean_object* v_r_439_; 
v_physical_boxed_436_ = lean_unbox_uint64(v_physical_434_);
lean_dec_ref(v_physical_434_);
v_kind_boxed_437_ = lean_unbox_uint64(v_kind_435_);
lean_dec_ref(v_kind_435_);
v_res_438_ = bastion_user_page_entry(v_physical_boxed_436_, v_kind_boxed_437_);
v_r_439_ = lean_box_uint64(v_res_438_);
return v_r_439_;
}
}
LEAN_EXPORT uint64_t bastion_syscall_opcode(uint64_t v_raw_440_){
_start:
{
uint64_t v___x_441_; uint8_t v___x_442_; 
v___x_441_ = 5ULL;
v___x_442_ = lean_uint64_dec_le(v_raw_440_, v___x_441_);
if (v___x_442_ == 0)
{
uint64_t v___x_443_; 
v___x_443_ = 255ULL;
return v___x_443_;
}
else
{
return v_raw_440_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_syscallOpcode___boxed(lean_object* v_raw_444_){
_start:
{
uint64_t v_raw_boxed_445_; uint64_t v_res_446_; lean_object* v_r_447_; 
v_raw_boxed_445_ = lean_unbox_uint64(v_raw_444_);
lean_dec_ref(v_raw_444_);
v_res_446_ = bastion_syscall_opcode(v_raw_boxed_445_);
v_r_447_ = lean_box_uint64(v_res_446_);
return v_r_447_;
}
}
LEAN_EXPORT uint8_t bastion_net_frame_len(uint64_t v_size_448_){
_start:
{
uint64_t v___x_449_; uint8_t v___x_450_; 
v___x_449_ = 14ULL;
v___x_450_ = lean_uint64_dec_le(v___x_449_, v_size_448_);
if (v___x_450_ == 0)
{
return v___x_450_;
}
else
{
uint64_t v___x_451_; uint8_t v___x_452_; 
v___x_451_ = 1514ULL;
v___x_452_ = lean_uint64_dec_le(v_size_448_, v___x_451_);
return v___x_452_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_netFrameLen___boxed(lean_object* v_size_453_){
_start:
{
uint64_t v_size_boxed_454_; uint8_t v_res_455_; lean_object* v_r_456_; 
v_size_boxed_454_ = lean_unbox_uint64(v_size_453_);
lean_dec_ref(v_size_453_);
v_res_455_ = bastion_net_frame_len(v_size_boxed_454_);
v_r_456_ = lean_box(v_res_455_);
return v_r_456_;
}
}
LEAN_EXPORT uint8_t bastion_net_ipv4(uint64_t v_size_457_, uint64_t v_available_458_, uint64_t v_fragment_459_, uint64_t v_ttl_460_, uint64_t v_protocol_461_, uint64_t v_version_462_){
_start:
{
uint8_t v___x_463_; uint8_t v___y_465_; uint8_t v___y_471_; uint64_t v___x_479_; uint8_t v___x_480_; 
v___x_463_ = lean_uint64_dec_le(v_size_457_, v_available_458_);
v___x_479_ = 69ULL;
v___x_480_ = lean_uint64_dec_eq(v_version_462_, v___x_479_);
if (v___x_480_ == 0)
{
v___y_471_ = v___x_480_;
goto v___jp_470_;
}
else
{
uint64_t v___x_481_; uint8_t v___x_482_; 
v___x_481_ = 17ULL;
v___x_482_ = lean_uint64_dec_eq(v_protocol_461_, v___x_481_);
v___y_471_ = v___x_482_;
goto v___jp_470_;
}
v___jp_464_:
{
if (v___y_465_ == 0)
{
return v___y_465_;
}
else
{
uint64_t v___x_466_; uint8_t v___x_467_; 
v___x_466_ = 28ULL;
v___x_467_ = lean_uint64_dec_le(v___x_466_, v_size_457_);
if (v___x_467_ == 0)
{
return v___x_467_;
}
else
{
if (v___x_463_ == 0)
{
return v___x_463_;
}
else
{
uint64_t v___x_468_; uint8_t v___x_469_; 
v___x_468_ = 1228ULL;
v___x_469_ = lean_uint64_dec_le(v_size_457_, v___x_468_);
return v___x_469_;
}
}
}
}
v___jp_470_:
{
if (v___y_471_ == 0)
{
return v___y_471_;
}
else
{
uint64_t v___x_472_; uint8_t v___x_473_; 
v___x_472_ = 0ULL;
v___x_473_ = lean_uint64_dec_lt(v___x_472_, v_ttl_460_);
if (v___x_473_ == 0)
{
return v___x_473_;
}
else
{
uint64_t v___x_474_; uint8_t v___x_475_; 
v___x_474_ = 255ULL;
v___x_475_ = lean_uint64_dec_le(v_ttl_460_, v___x_474_);
if (v___x_475_ == 0)
{
return v___x_475_;
}
else
{
uint8_t v___x_476_; 
v___x_476_ = lean_uint64_dec_eq(v_fragment_459_, v___x_472_);
if (v___x_476_ == 0)
{
uint64_t v___x_477_; uint8_t v___x_478_; 
v___x_477_ = 16384ULL;
v___x_478_ = lean_uint64_dec_eq(v_fragment_459_, v___x_477_);
v___y_465_ = v___x_478_;
goto v___jp_464_;
}
else
{
v___y_465_ = v___x_476_;
goto v___jp_464_;
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_netIpv4___boxed(lean_object* v_size_483_, lean_object* v_available_484_, lean_object* v_fragment_485_, lean_object* v_ttl_486_, lean_object* v_protocol_487_, lean_object* v_version_488_){
_start:
{
uint64_t v_size_boxed_489_; uint64_t v_available_boxed_490_; uint64_t v_fragment_boxed_491_; uint64_t v_ttl_boxed_492_; uint64_t v_protocol_boxed_493_; uint64_t v_version_boxed_494_; uint8_t v_res_495_; lean_object* v_r_496_; 
v_size_boxed_489_ = lean_unbox_uint64(v_size_483_);
lean_dec_ref(v_size_483_);
v_available_boxed_490_ = lean_unbox_uint64(v_available_484_);
lean_dec_ref(v_available_484_);
v_fragment_boxed_491_ = lean_unbox_uint64(v_fragment_485_);
lean_dec_ref(v_fragment_485_);
v_ttl_boxed_492_ = lean_unbox_uint64(v_ttl_486_);
lean_dec_ref(v_ttl_486_);
v_protocol_boxed_493_ = lean_unbox_uint64(v_protocol_487_);
lean_dec_ref(v_protocol_487_);
v_version_boxed_494_ = lean_unbox_uint64(v_version_488_);
lean_dec_ref(v_version_488_);
v_res_495_ = bastion_net_ipv4(v_size_boxed_489_, v_available_boxed_490_, v_fragment_boxed_491_, v_ttl_boxed_492_, v_protocol_boxed_493_, v_version_boxed_494_);
v_r_496_ = lean_box(v_res_495_);
return v_r_496_;
}
}
LEAN_EXPORT uint8_t bastion_net_udp(uint64_t v_size_497_, uint64_t v_available_498_, uint64_t v_destination_499_, uint64_t v_bound_500_){
_start:
{
uint8_t v___y_502_; uint64_t v___x_509_; uint8_t v___x_510_; 
v___x_509_ = 8ULL;
v___x_510_ = lean_uint64_dec_le(v___x_509_, v_size_497_);
if (v___x_510_ == 0)
{
v___y_502_ = v___x_510_;
goto v___jp_501_;
}
else
{
uint64_t v___x_511_; uint8_t v___x_512_; 
v___x_511_ = 1208ULL;
v___x_512_ = lean_uint64_dec_le(v_size_497_, v___x_511_);
v___y_502_ = v___x_512_;
goto v___jp_501_;
}
v___jp_501_:
{
if (v___y_502_ == 0)
{
return v___y_502_;
}
else
{
uint8_t v___x_503_; 
v___x_503_ = lean_uint64_dec_eq(v_size_497_, v_available_498_);
if (v___x_503_ == 0)
{
return v___x_503_;
}
else
{
uint64_t v___x_504_; uint8_t v___x_505_; 
v___x_504_ = 0ULL;
v___x_505_ = lean_uint64_dec_lt(v___x_504_, v_bound_500_);
if (v___x_505_ == 0)
{
return v___x_505_;
}
else
{
uint64_t v___x_506_; uint8_t v___x_507_; 
v___x_506_ = 65535ULL;
v___x_507_ = lean_uint64_dec_le(v_bound_500_, v___x_506_);
if (v___x_507_ == 0)
{
return v___x_507_;
}
else
{
uint8_t v___x_508_; 
v___x_508_ = lean_uint64_dec_eq(v_destination_499_, v_bound_500_);
return v___x_508_;
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_netUdp___boxed(lean_object* v_size_513_, lean_object* v_available_514_, lean_object* v_destination_515_, lean_object* v_bound_516_){
_start:
{
uint64_t v_size_boxed_517_; uint64_t v_available_boxed_518_; uint64_t v_destination_boxed_519_; uint64_t v_bound_boxed_520_; uint8_t v_res_521_; lean_object* v_r_522_; 
v_size_boxed_517_ = lean_unbox_uint64(v_size_513_);
lean_dec_ref(v_size_513_);
v_available_boxed_518_ = lean_unbox_uint64(v_available_514_);
lean_dec_ref(v_available_514_);
v_destination_boxed_519_ = lean_unbox_uint64(v_destination_515_);
lean_dec_ref(v_destination_515_);
v_bound_boxed_520_ = lean_unbox_uint64(v_bound_516_);
lean_dec_ref(v_bound_516_);
v_res_521_ = bastion_net_udp(v_size_boxed_517_, v_available_boxed_518_, v_destination_boxed_519_, v_bound_boxed_520_);
v_r_522_ = lean_box(v_res_521_);
return v_r_522_;
}
}
LEAN_EXPORT uint8_t bastion_net_budget(uint64_t v_processed_523_){
_start:
{
uint64_t v___x_524_; uint8_t v___x_525_; 
v___x_524_ = 8ULL;
v___x_525_ = lean_uint64_dec_lt(v_processed_523_, v___x_524_);
return v___x_525_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_netBudget___boxed(lean_object* v_processed_526_){
_start:
{
uint64_t v_processed_boxed_527_; uint8_t v_res_528_; lean_object* v_r_529_; 
v_processed_boxed_527_ = lean_unbox_uint64(v_processed_526_);
lean_dec_ref(v_processed_526_);
v_res_528_ = bastion_net_budget(v_processed_boxed_527_);
v_r_529_ = lean_box(v_res_528_);
return v_r_529_;
}
}
LEAN_EXPORT uint8_t bastion_relay_ingress(uint64_t v_relay_530_, uint64_t v_role_531_, uint64_t v_opcode_532_, uint64_t v_sequence_533_, uint64_t v_previous_534_, uint64_t v_size_535_){
_start:
{
uint8_t v___y_537_; uint8_t v___y_545_; uint8_t v___x_548_; uint8_t v___y_550_; uint8_t v___y_557_; uint64_t v___x_562_; uint8_t v___x_563_; 
v___x_548_ = lean_uint64_dec_lt(v_previous_534_, v_sequence_533_);
v___x_562_ = 1ULL;
v___x_563_ = lean_uint64_dec_le(v___x_562_, v_relay_530_);
if (v___x_563_ == 0)
{
v___y_557_ = v___x_563_;
goto v___jp_556_;
}
else
{
uint64_t v___x_564_; uint8_t v___x_565_; 
v___x_564_ = 8ULL;
v___x_565_ = lean_uint64_dec_le(v_relay_530_, v___x_564_);
v___y_557_ = v___x_565_;
goto v___jp_556_;
}
v___jp_536_:
{
if (v___y_537_ == 0)
{
return v___y_537_;
}
else
{
uint64_t v___x_538_; uint8_t v___x_539_; 
v___x_538_ = 0ULL;
v___x_539_ = lean_uint64_dec_eq(v_size_535_, v___x_538_);
return v___x_539_;
}
}
v___jp_540_:
{
uint64_t v___x_541_; uint8_t v___x_542_; 
v___x_541_ = 2ULL;
v___x_542_ = lean_uint64_dec_eq(v_role_531_, v___x_541_);
if (v___x_542_ == 0)
{
v___y_537_ = v___x_542_;
goto v___jp_536_;
}
else
{
uint8_t v___x_543_; 
v___x_543_ = lean_uint64_dec_eq(v_opcode_532_, v___x_541_);
v___y_537_ = v___x_543_;
goto v___jp_536_;
}
}
v___jp_544_:
{
if (v___y_545_ == 0)
{
goto v___jp_540_;
}
else
{
uint64_t v___x_546_; uint8_t v___x_547_; 
v___x_546_ = 176ULL;
v___x_547_ = lean_uint64_dec_eq(v_size_535_, v___x_546_);
if (v___x_547_ == 0)
{
goto v___jp_540_;
}
else
{
return v___x_547_;
}
}
}
v___jp_549_:
{
if (v___y_550_ == 0)
{
return v___y_550_;
}
else
{
if (v___x_548_ == 0)
{
return v___x_548_;
}
else
{
uint64_t v___x_551_; uint8_t v___x_552_; 
v___x_551_ = 1048576ULL;
v___x_552_ = lean_uint64_dec_lt(v_sequence_533_, v___x_551_);
if (v___x_552_ == 0)
{
return v___x_552_;
}
else
{
uint64_t v___x_553_; uint8_t v___x_554_; 
v___x_553_ = 1ULL;
v___x_554_ = lean_uint64_dec_eq(v_role_531_, v___x_553_);
if (v___x_554_ == 0)
{
v___y_545_ = v___x_554_;
goto v___jp_544_;
}
else
{
uint8_t v___x_555_; 
v___x_555_ = lean_uint64_dec_eq(v_opcode_532_, v___x_553_);
v___y_545_ = v___x_555_;
goto v___jp_544_;
}
}
}
}
}
v___jp_556_:
{
if (v___y_557_ == 0)
{
return v___y_557_;
}
else
{
uint64_t v___x_558_; uint8_t v___x_559_; 
v___x_558_ = 1ULL;
v___x_559_ = lean_uint64_dec_eq(v_role_531_, v___x_558_);
if (v___x_559_ == 0)
{
uint64_t v___x_560_; uint8_t v___x_561_; 
v___x_560_ = 2ULL;
v___x_561_ = lean_uint64_dec_eq(v_role_531_, v___x_560_);
v___y_550_ = v___x_561_;
goto v___jp_549_;
}
else
{
v___y_550_ = v___x_559_;
goto v___jp_549_;
}
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_relayIngress___boxed(lean_object* v_relay_566_, lean_object* v_role_567_, lean_object* v_opcode_568_, lean_object* v_sequence_569_, lean_object* v_previous_570_, lean_object* v_size_571_){
_start:
{
uint64_t v_relay_boxed_572_; uint64_t v_role_boxed_573_; uint64_t v_opcode_boxed_574_; uint64_t v_sequence_boxed_575_; uint64_t v_previous_boxed_576_; uint64_t v_size_boxed_577_; uint8_t v_res_578_; lean_object* v_r_579_; 
v_relay_boxed_572_ = lean_unbox_uint64(v_relay_566_);
lean_dec_ref(v_relay_566_);
v_role_boxed_573_ = lean_unbox_uint64(v_role_567_);
lean_dec_ref(v_role_567_);
v_opcode_boxed_574_ = lean_unbox_uint64(v_opcode_568_);
lean_dec_ref(v_opcode_568_);
v_sequence_boxed_575_ = lean_unbox_uint64(v_sequence_569_);
lean_dec_ref(v_sequence_569_);
v_previous_boxed_576_ = lean_unbox_uint64(v_previous_570_);
lean_dec_ref(v_previous_570_);
v_size_boxed_577_ = lean_unbox_uint64(v_size_571_);
lean_dec_ref(v_size_571_);
v_res_578_ = bastion_relay_ingress(v_relay_boxed_572_, v_role_boxed_573_, v_opcode_boxed_574_, v_sequence_boxed_575_, v_previous_boxed_576_, v_size_boxed_577_);
v_r_579_ = lean_box(v_res_578_);
return v_r_579_;
}
}
LEAN_EXPORT uint64_t bastion_field_share(uint64_t v_secret_580_, uint64_t v_a_581_, uint64_t v_b_582_, uint64_t v_c_583_, uint64_t v_x_584_){
_start:
{
uint8_t v___y_586_; uint64_t v___x_609_; uint8_t v___x_610_; 
v___x_609_ = 255ULL;
v___x_610_ = lean_uint64_dec_lt(v___x_609_, v_secret_580_);
if (v___x_610_ == 0)
{
uint64_t v___x_611_; uint8_t v___x_612_; 
v___x_611_ = 256ULL;
v___x_612_ = lean_uint64_dec_lt(v___x_611_, v_a_581_);
v___y_586_ = v___x_612_;
goto v___jp_585_;
}
else
{
v___y_586_ = v___x_610_;
goto v___jp_585_;
}
v___jp_585_:
{
if (v___y_586_ == 0)
{
uint64_t v___x_587_; uint8_t v___x_588_; 
v___x_587_ = 256ULL;
v___x_588_ = lean_uint64_dec_lt(v___x_587_, v_b_582_);
if (v___x_588_ == 0)
{
uint8_t v___x_589_; 
v___x_589_ = lean_uint64_dec_lt(v___x_587_, v_c_583_);
if (v___x_589_ == 0)
{
uint64_t v___x_590_; uint8_t v___x_591_; 
v___x_590_ = 1ULL;
v___x_591_ = lean_uint64_dec_lt(v_x_584_, v___x_590_);
if (v___x_591_ == 0)
{
uint64_t v___x_592_; uint8_t v___x_593_; 
v___x_592_ = 8ULL;
v___x_593_ = lean_uint64_dec_lt(v___x_592_, v_x_584_);
if (v___x_593_ == 0)
{
uint64_t v___x_594_; uint64_t v___x_595_; uint64_t v___x_596_; uint64_t v___x_597_; uint64_t v___x_598_; uint64_t v___x_599_; uint64_t v___x_600_; uint64_t v___x_601_; uint64_t v___x_602_; uint64_t v___x_603_; 
v___x_594_ = lean_uint64_mul(v_c_583_, v_x_584_);
v___x_595_ = lean_uint64_add(v___x_594_, v_b_582_);
v___x_596_ = 257ULL;
v___x_597_ = lean_uint64_mod(v___x_595_, v___x_596_);
v___x_598_ = lean_uint64_mul(v___x_597_, v_x_584_);
v___x_599_ = lean_uint64_add(v___x_598_, v_a_581_);
v___x_600_ = lean_uint64_mod(v___x_599_, v___x_596_);
v___x_601_ = lean_uint64_mul(v___x_600_, v_x_584_);
v___x_602_ = lean_uint64_add(v___x_601_, v_secret_580_);
v___x_603_ = lean_uint64_mod(v___x_602_, v___x_596_);
return v___x_603_;
}
else
{
uint64_t v___x_604_; 
v___x_604_ = 257ULL;
return v___x_604_;
}
}
else
{
uint64_t v___x_605_; 
v___x_605_ = 257ULL;
return v___x_605_;
}
}
else
{
uint64_t v___x_606_; 
v___x_606_ = 257ULL;
return v___x_606_;
}
}
else
{
uint64_t v___x_607_; 
v___x_607_ = 257ULL;
return v___x_607_;
}
}
else
{
uint64_t v___x_608_; 
v___x_608_ = 257ULL;
return v___x_608_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldShare___boxed(lean_object* v_secret_613_, lean_object* v_a_614_, lean_object* v_b_615_, lean_object* v_c_616_, lean_object* v_x_617_){
_start:
{
uint64_t v_secret_boxed_618_; uint64_t v_a_boxed_619_; uint64_t v_b_boxed_620_; uint64_t v_c_boxed_621_; uint64_t v_x_boxed_622_; uint64_t v_res_623_; lean_object* v_r_624_; 
v_secret_boxed_618_ = lean_unbox_uint64(v_secret_613_);
lean_dec_ref(v_secret_613_);
v_a_boxed_619_ = lean_unbox_uint64(v_a_614_);
lean_dec_ref(v_a_614_);
v_b_boxed_620_ = lean_unbox_uint64(v_b_615_);
lean_dec_ref(v_b_615_);
v_c_boxed_621_ = lean_unbox_uint64(v_c_616_);
lean_dec_ref(v_c_616_);
v_x_boxed_622_ = lean_unbox_uint64(v_x_617_);
lean_dec_ref(v_x_617_);
v_res_623_ = bastion_field_share(v_secret_boxed_618_, v_a_boxed_619_, v_b_boxed_620_, v_c_boxed_621_, v_x_boxed_622_);
v_r_624_ = lean_box_uint64(v_res_623_);
return v_r_624_;
}
}
LEAN_EXPORT uint64_t l_Bastion_Runtime_fieldInv(uint64_t v_x_625_){
_start:
{
uint64_t v___x_626_; uint64_t v_a_627_; uint64_t v___x_628_; uint64_t v_b_629_; uint64_t v___x_630_; uint64_t v_c_631_; uint64_t v___x_632_; uint64_t v_d_633_; uint64_t v___x_634_; uint64_t v_e_635_; uint64_t v___x_636_; uint64_t v_f_637_; uint64_t v___x_638_; uint64_t v_g_639_; uint64_t v___x_640_; uint64_t v_h_641_; uint64_t v___x_642_; uint64_t v___x_643_; uint64_t v___x_644_; uint64_t v___x_645_; uint64_t v___x_646_; uint64_t v___x_647_; uint64_t v___x_648_; uint64_t v___x_649_; uint64_t v___x_650_; uint64_t v___x_651_; uint64_t v___x_652_; uint64_t v___x_653_; uint64_t v___x_654_; uint64_t v___x_655_; 
v___x_626_ = 257ULL;
v_a_627_ = lean_uint64_mod(v_x_625_, v___x_626_);
v___x_628_ = lean_uint64_mul(v_a_627_, v_a_627_);
v_b_629_ = lean_uint64_mod(v___x_628_, v___x_626_);
v___x_630_ = lean_uint64_mul(v_b_629_, v_b_629_);
v_c_631_ = lean_uint64_mod(v___x_630_, v___x_626_);
v___x_632_ = lean_uint64_mul(v_c_631_, v_c_631_);
v_d_633_ = lean_uint64_mod(v___x_632_, v___x_626_);
v___x_634_ = lean_uint64_mul(v_d_633_, v_d_633_);
v_e_635_ = lean_uint64_mod(v___x_634_, v___x_626_);
v___x_636_ = lean_uint64_mul(v_e_635_, v_e_635_);
v_f_637_ = lean_uint64_mod(v___x_636_, v___x_626_);
v___x_638_ = lean_uint64_mul(v_f_637_, v_f_637_);
v_g_639_ = lean_uint64_mod(v___x_638_, v___x_626_);
v___x_640_ = lean_uint64_mul(v_g_639_, v_g_639_);
v_h_641_ = lean_uint64_mod(v___x_640_, v___x_626_);
v___x_642_ = lean_uint64_mul(v_a_627_, v_b_629_);
v___x_643_ = lean_uint64_mod(v___x_642_, v___x_626_);
v___x_644_ = lean_uint64_mul(v___x_643_, v_c_631_);
v___x_645_ = lean_uint64_mod(v___x_644_, v___x_626_);
v___x_646_ = lean_uint64_mul(v___x_645_, v_d_633_);
v___x_647_ = lean_uint64_mod(v___x_646_, v___x_626_);
v___x_648_ = lean_uint64_mul(v___x_647_, v_e_635_);
v___x_649_ = lean_uint64_mod(v___x_648_, v___x_626_);
v___x_650_ = lean_uint64_mul(v___x_649_, v_f_637_);
v___x_651_ = lean_uint64_mod(v___x_650_, v___x_626_);
v___x_652_ = lean_uint64_mul(v___x_651_, v_g_639_);
v___x_653_ = lean_uint64_mod(v___x_652_, v___x_626_);
v___x_654_ = lean_uint64_mul(v___x_653_, v_h_641_);
v___x_655_ = lean_uint64_mod(v___x_654_, v___x_626_);
return v___x_655_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldInv___boxed(lean_object* v_x_656_){
_start:
{
uint64_t v_x_boxed_657_; uint64_t v_res_658_; lean_object* v_r_659_; 
v_x_boxed_657_ = lean_unbox_uint64(v_x_656_);
lean_dec_ref(v_x_656_);
v_res_658_ = l_Bastion_Runtime_fieldInv(v_x_boxed_657_);
v_r_659_ = lean_box_uint64(v_res_658_);
return v_r_659_;
}
}
LEAN_EXPORT uint64_t l_Bastion_Runtime_weight(uint64_t v_x_660_, uint64_t v_a_661_, uint64_t v_b_662_, uint64_t v_c_663_){
_start:
{
uint64_t v___x_664_; uint64_t v___x_665_; uint64_t v___x_666_; uint64_t v___x_667_; uint64_t v___x_668_; uint64_t v___x_669_; uint64_t v___x_670_; uint64_t v_numerator_671_; uint64_t v___x_672_; uint64_t v___x_673_; uint64_t v___x_674_; uint64_t v___x_675_; uint64_t v___x_676_; uint64_t v___x_677_; uint64_t v___x_678_; uint64_t v_denominator_679_; uint64_t v_a_680_; uint64_t v___x_681_; uint64_t v_b_682_; uint64_t v___x_683_; uint64_t v_c_684_; uint64_t v___x_685_; uint64_t v_d_686_; uint64_t v___x_687_; uint64_t v_e_688_; uint64_t v___x_689_; uint64_t v_f_690_; uint64_t v___x_691_; uint64_t v_g_692_; uint64_t v___x_693_; uint64_t v_h_694_; uint64_t v___x_695_; uint64_t v___x_696_; uint64_t v___x_697_; uint64_t v___x_698_; uint64_t v___x_699_; uint64_t v___x_700_; uint64_t v___x_701_; uint64_t v___x_702_; uint64_t v___x_703_; uint64_t v___x_704_; uint64_t v___x_705_; uint64_t v___x_706_; uint64_t v___x_707_; uint64_t v___x_708_; uint64_t v___x_709_; uint64_t v___x_710_; 
v___x_664_ = 257ULL;
v___x_665_ = lean_uint64_sub(v___x_664_, v_a_661_);
v___x_666_ = lean_uint64_sub(v___x_664_, v_b_662_);
v___x_667_ = lean_uint64_mul(v___x_665_, v___x_666_);
v___x_668_ = lean_uint64_mod(v___x_667_, v___x_664_);
v___x_669_ = lean_uint64_sub(v___x_664_, v_c_663_);
v___x_670_ = lean_uint64_mul(v___x_668_, v___x_669_);
v_numerator_671_ = lean_uint64_mod(v___x_670_, v___x_664_);
v___x_672_ = lean_uint64_add(v_x_660_, v___x_664_);
v___x_673_ = lean_uint64_sub(v___x_672_, v_a_661_);
v___x_674_ = lean_uint64_sub(v___x_672_, v_b_662_);
v___x_675_ = lean_uint64_mul(v___x_673_, v___x_674_);
v___x_676_ = lean_uint64_mod(v___x_675_, v___x_664_);
v___x_677_ = lean_uint64_sub(v___x_672_, v_c_663_);
v___x_678_ = lean_uint64_mul(v___x_676_, v___x_677_);
v_denominator_679_ = lean_uint64_mod(v___x_678_, v___x_664_);
v_a_680_ = lean_uint64_mod(v_denominator_679_, v___x_664_);
v___x_681_ = lean_uint64_mul(v_a_680_, v_a_680_);
v_b_682_ = lean_uint64_mod(v___x_681_, v___x_664_);
v___x_683_ = lean_uint64_mul(v_b_682_, v_b_682_);
v_c_684_ = lean_uint64_mod(v___x_683_, v___x_664_);
v___x_685_ = lean_uint64_mul(v_c_684_, v_c_684_);
v_d_686_ = lean_uint64_mod(v___x_685_, v___x_664_);
v___x_687_ = lean_uint64_mul(v_d_686_, v_d_686_);
v_e_688_ = lean_uint64_mod(v___x_687_, v___x_664_);
v___x_689_ = lean_uint64_mul(v_e_688_, v_e_688_);
v_f_690_ = lean_uint64_mod(v___x_689_, v___x_664_);
v___x_691_ = lean_uint64_mul(v_f_690_, v_f_690_);
v_g_692_ = lean_uint64_mod(v___x_691_, v___x_664_);
v___x_693_ = lean_uint64_mul(v_g_692_, v_g_692_);
v_h_694_ = lean_uint64_mod(v___x_693_, v___x_664_);
v___x_695_ = lean_uint64_mul(v_a_680_, v_b_682_);
v___x_696_ = lean_uint64_mod(v___x_695_, v___x_664_);
v___x_697_ = lean_uint64_mul(v___x_696_, v_c_684_);
v___x_698_ = lean_uint64_mod(v___x_697_, v___x_664_);
v___x_699_ = lean_uint64_mul(v___x_698_, v_d_686_);
v___x_700_ = lean_uint64_mod(v___x_699_, v___x_664_);
v___x_701_ = lean_uint64_mul(v___x_700_, v_e_688_);
v___x_702_ = lean_uint64_mod(v___x_701_, v___x_664_);
v___x_703_ = lean_uint64_mul(v___x_702_, v_f_690_);
v___x_704_ = lean_uint64_mod(v___x_703_, v___x_664_);
v___x_705_ = lean_uint64_mul(v___x_704_, v_g_692_);
v___x_706_ = lean_uint64_mod(v___x_705_, v___x_664_);
v___x_707_ = lean_uint64_mul(v___x_706_, v_h_694_);
v___x_708_ = lean_uint64_mod(v___x_707_, v___x_664_);
v___x_709_ = lean_uint64_mul(v_numerator_671_, v___x_708_);
v___x_710_ = lean_uint64_mod(v___x_709_, v___x_664_);
return v___x_710_;
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_weight___boxed(lean_object* v_x_711_, lean_object* v_a_712_, lean_object* v_b_713_, lean_object* v_c_714_){
_start:
{
uint64_t v_x_boxed_715_; uint64_t v_a_boxed_716_; uint64_t v_b_boxed_717_; uint64_t v_c_boxed_718_; uint64_t v_res_719_; lean_object* v_r_720_; 
v_x_boxed_715_ = lean_unbox_uint64(v_x_711_);
lean_dec_ref(v_x_711_);
v_a_boxed_716_ = lean_unbox_uint64(v_a_712_);
lean_dec_ref(v_a_712_);
v_b_boxed_717_ = lean_unbox_uint64(v_b_713_);
lean_dec_ref(v_b_713_);
v_c_boxed_718_ = lean_unbox_uint64(v_c_714_);
lean_dec_ref(v_c_714_);
v_res_719_ = l_Bastion_Runtime_weight(v_x_boxed_715_, v_a_boxed_716_, v_b_boxed_717_, v_c_boxed_718_);
v_r_720_ = lean_box_uint64(v_res_719_);
return v_r_720_;
}
}
LEAN_EXPORT uint64_t bastion_field_reconstruct(uint64_t v_points_721_, uint64_t v_y0_722_, uint64_t v_y1_723_, uint64_t v_y2_724_, uint64_t v_y3_725_){
_start:
{
uint64_t v___x_726_; uint64_t v_a_727_; uint64_t v___x_728_; uint64_t v___x_729_; uint64_t v_b_730_; uint64_t v___x_731_; uint64_t v___x_732_; uint64_t v_c_733_; uint64_t v___x_734_; uint64_t v___x_735_; uint64_t v_d_736_; uint8_t v___x_737_; uint8_t v___y_739_; uint8_t v___x_934_; uint8_t v___x_935_; uint8_t v___x_936_; uint8_t v___y_938_; uint64_t v___x_941_; uint8_t v___x_942_; 
v___x_726_ = 255ULL;
v_a_727_ = lean_uint64_land(v_points_721_, v___x_726_);
v___x_728_ = 8ULL;
v___x_729_ = lean_uint64_shift_right(v_points_721_, v___x_728_);
v_b_730_ = lean_uint64_land(v___x_729_, v___x_726_);
v___x_731_ = 16ULL;
v___x_732_ = lean_uint64_shift_right(v_points_721_, v___x_731_);
v_c_733_ = lean_uint64_land(v___x_732_, v___x_726_);
v___x_734_ = 24ULL;
v___x_735_ = lean_uint64_shift_right(v_points_721_, v___x_734_);
v_d_736_ = lean_uint64_land(v___x_735_, v___x_726_);
v___x_737_ = lean_uint64_dec_lt(v_c_733_, v_d_736_);
v___x_934_ = lean_uint64_dec_lt(v_b_730_, v_c_733_);
v___x_935_ = lean_uint64_dec_lt(v_a_727_, v_b_730_);
v___x_936_ = lean_uint64_dec_lt(v___x_728_, v_d_736_);
v___x_941_ = 4294967295ULL;
v___x_942_ = lean_uint64_dec_lt(v___x_941_, v_points_721_);
if (v___x_942_ == 0)
{
uint64_t v___x_943_; uint8_t v___x_944_; 
v___x_943_ = 1ULL;
v___x_944_ = lean_uint64_dec_lt(v_a_727_, v___x_943_);
v___y_938_ = v___x_944_;
goto v___jp_937_;
}
else
{
v___y_938_ = v___x_942_;
goto v___jp_937_;
}
v___jp_738_:
{
if (v___y_739_ == 0)
{
uint64_t v___x_740_; 
v___x_740_ = 257ULL;
return v___x_740_;
}
else
{
if (v___x_737_ == 0)
{
uint64_t v___x_741_; 
v___x_741_ = 257ULL;
return v___x_741_;
}
else
{
uint64_t v___x_742_; uint8_t v___x_743_; 
v___x_742_ = 256ULL;
v___x_743_ = lean_uint64_dec_lt(v___x_742_, v_y0_722_);
if (v___x_743_ == 0)
{
uint8_t v___x_744_; 
v___x_744_ = lean_uint64_dec_lt(v___x_742_, v_y1_723_);
if (v___x_744_ == 0)
{
uint8_t v___x_745_; 
v___x_745_ = lean_uint64_dec_lt(v___x_742_, v_y2_724_);
if (v___x_745_ == 0)
{
uint8_t v___x_746_; 
v___x_746_ = lean_uint64_dec_lt(v___x_742_, v_y3_725_);
if (v___x_746_ == 0)
{
uint64_t v___x_747_; uint64_t v___x_748_; uint64_t v___x_749_; uint64_t v___x_750_; uint64_t v___x_751_; uint64_t v___x_752_; uint64_t v___x_753_; uint64_t v_numerator_754_; uint64_t v___x_755_; uint64_t v___x_756_; uint64_t v___x_757_; uint64_t v___x_758_; uint64_t v___x_759_; uint64_t v___x_760_; uint64_t v___x_761_; uint64_t v_denominator_762_; uint64_t v_a_763_; uint64_t v___x_764_; uint64_t v_b_765_; uint64_t v___x_766_; uint64_t v_c_767_; uint64_t v___x_768_; uint64_t v_d_769_; uint64_t v___x_770_; uint64_t v_e_771_; uint64_t v___x_772_; uint64_t v_f_773_; uint64_t v___x_774_; uint64_t v_g_775_; uint64_t v___x_776_; uint64_t v_h_777_; uint64_t v___x_778_; uint64_t v___x_779_; uint64_t v___x_780_; uint64_t v___x_781_; uint64_t v___x_782_; uint64_t v___x_783_; uint64_t v___x_784_; uint64_t v___x_785_; uint64_t v___x_786_; uint64_t v___x_787_; uint64_t v___x_788_; uint64_t v___x_789_; uint64_t v___x_790_; uint64_t v___x_791_; uint64_t v___x_792_; uint64_t v___x_793_; uint64_t v___x_794_; uint64_t v___x_795_; uint64_t v___x_796_; uint64_t v___x_797_; uint64_t v___x_798_; uint64_t v_numerator_799_; uint64_t v___x_800_; uint64_t v___x_801_; uint64_t v___x_802_; uint64_t v___x_803_; uint64_t v___x_804_; uint64_t v___x_805_; uint64_t v___x_806_; uint64_t v_denominator_807_; uint64_t v_a_808_; uint64_t v___x_809_; uint64_t v_b_810_; uint64_t v___x_811_; uint64_t v_c_812_; uint64_t v___x_813_; uint64_t v_d_814_; uint64_t v___x_815_; uint64_t v_e_816_; uint64_t v___x_817_; uint64_t v_f_818_; uint64_t v___x_819_; uint64_t v_g_820_; uint64_t v___x_821_; uint64_t v_h_822_; uint64_t v___x_823_; uint64_t v___x_824_; uint64_t v___x_825_; uint64_t v___x_826_; uint64_t v___x_827_; uint64_t v___x_828_; uint64_t v___x_829_; uint64_t v___x_830_; uint64_t v___x_831_; uint64_t v___x_832_; uint64_t v___x_833_; uint64_t v___x_834_; uint64_t v___x_835_; uint64_t v___x_836_; uint64_t v___x_837_; uint64_t v___x_838_; uint64_t v___x_839_; uint64_t v___x_840_; uint64_t v___x_841_; uint64_t v___x_842_; uint64_t v___x_843_; uint64_t v_numerator_844_; uint64_t v___x_845_; uint64_t v___x_846_; uint64_t v___x_847_; uint64_t v___x_848_; uint64_t v___x_849_; uint64_t v___x_850_; uint64_t v___x_851_; uint64_t v_denominator_852_; uint64_t v_a_853_; uint64_t v___x_854_; uint64_t v_b_855_; uint64_t v___x_856_; uint64_t v_c_857_; uint64_t v___x_858_; uint64_t v_d_859_; uint64_t v___x_860_; uint64_t v_e_861_; uint64_t v___x_862_; uint64_t v_f_863_; uint64_t v___x_864_; uint64_t v_g_865_; uint64_t v___x_866_; uint64_t v_h_867_; uint64_t v___x_868_; uint64_t v___x_869_; uint64_t v___x_870_; uint64_t v___x_871_; uint64_t v___x_872_; uint64_t v___x_873_; uint64_t v___x_874_; uint64_t v___x_875_; uint64_t v___x_876_; uint64_t v___x_877_; uint64_t v___x_878_; uint64_t v___x_879_; uint64_t v___x_880_; uint64_t v___x_881_; uint64_t v___x_882_; uint64_t v___x_883_; uint64_t v___x_884_; uint64_t v___x_885_; uint64_t v___x_886_; uint64_t v_numerator_887_; uint64_t v___x_888_; uint64_t v___x_889_; uint64_t v___x_890_; uint64_t v___x_891_; uint64_t v___x_892_; uint64_t v___x_893_; uint64_t v___x_894_; uint64_t v_denominator_895_; uint64_t v_a_896_; uint64_t v___x_897_; uint64_t v_b_898_; uint64_t v___x_899_; uint64_t v_c_900_; uint64_t v___x_901_; uint64_t v_d_902_; uint64_t v___x_903_; uint64_t v_e_904_; uint64_t v___x_905_; uint64_t v_f_906_; uint64_t v___x_907_; uint64_t v_g_908_; uint64_t v___x_909_; uint64_t v_h_910_; uint64_t v___x_911_; uint64_t v___x_912_; uint64_t v___x_913_; uint64_t v___x_914_; uint64_t v___x_915_; uint64_t v___x_916_; uint64_t v___x_917_; uint64_t v___x_918_; uint64_t v___x_919_; uint64_t v___x_920_; uint64_t v___x_921_; uint64_t v___x_922_; uint64_t v___x_923_; uint64_t v___x_924_; uint64_t v___x_925_; uint64_t v___x_926_; uint64_t v___x_927_; uint64_t v___x_928_; uint64_t v___x_929_; 
v___x_747_ = 257ULL;
v___x_748_ = lean_uint64_sub(v___x_747_, v_b_730_);
v___x_749_ = lean_uint64_sub(v___x_747_, v_c_733_);
v___x_750_ = lean_uint64_mul(v___x_748_, v___x_749_);
v___x_751_ = lean_uint64_mod(v___x_750_, v___x_747_);
v___x_752_ = lean_uint64_sub(v___x_747_, v_d_736_);
v___x_753_ = lean_uint64_mul(v___x_751_, v___x_752_);
v_numerator_754_ = lean_uint64_mod(v___x_753_, v___x_747_);
v___x_755_ = lean_uint64_add(v_a_727_, v___x_747_);
v___x_756_ = lean_uint64_sub(v___x_755_, v_b_730_);
v___x_757_ = lean_uint64_sub(v___x_755_, v_c_733_);
v___x_758_ = lean_uint64_mul(v___x_756_, v___x_757_);
v___x_759_ = lean_uint64_mod(v___x_758_, v___x_747_);
v___x_760_ = lean_uint64_sub(v___x_755_, v_d_736_);
v___x_761_ = lean_uint64_mul(v___x_759_, v___x_760_);
v_denominator_762_ = lean_uint64_mod(v___x_761_, v___x_747_);
v_a_763_ = lean_uint64_mod(v_denominator_762_, v___x_747_);
v___x_764_ = lean_uint64_mul(v_a_763_, v_a_763_);
v_b_765_ = lean_uint64_mod(v___x_764_, v___x_747_);
v___x_766_ = lean_uint64_mul(v_b_765_, v_b_765_);
v_c_767_ = lean_uint64_mod(v___x_766_, v___x_747_);
v___x_768_ = lean_uint64_mul(v_c_767_, v_c_767_);
v_d_769_ = lean_uint64_mod(v___x_768_, v___x_747_);
v___x_770_ = lean_uint64_mul(v_d_769_, v_d_769_);
v_e_771_ = lean_uint64_mod(v___x_770_, v___x_747_);
v___x_772_ = lean_uint64_mul(v_e_771_, v_e_771_);
v_f_773_ = lean_uint64_mod(v___x_772_, v___x_747_);
v___x_774_ = lean_uint64_mul(v_f_773_, v_f_773_);
v_g_775_ = lean_uint64_mod(v___x_774_, v___x_747_);
v___x_776_ = lean_uint64_mul(v_g_775_, v_g_775_);
v_h_777_ = lean_uint64_mod(v___x_776_, v___x_747_);
v___x_778_ = lean_uint64_mul(v_a_763_, v_b_765_);
v___x_779_ = lean_uint64_mod(v___x_778_, v___x_747_);
v___x_780_ = lean_uint64_mul(v___x_779_, v_c_767_);
v___x_781_ = lean_uint64_mod(v___x_780_, v___x_747_);
v___x_782_ = lean_uint64_mul(v___x_781_, v_d_769_);
v___x_783_ = lean_uint64_mod(v___x_782_, v___x_747_);
v___x_784_ = lean_uint64_mul(v___x_783_, v_e_771_);
v___x_785_ = lean_uint64_mod(v___x_784_, v___x_747_);
v___x_786_ = lean_uint64_mul(v___x_785_, v_f_773_);
v___x_787_ = lean_uint64_mod(v___x_786_, v___x_747_);
v___x_788_ = lean_uint64_mul(v___x_787_, v_g_775_);
v___x_789_ = lean_uint64_mod(v___x_788_, v___x_747_);
v___x_790_ = lean_uint64_mul(v___x_789_, v_h_777_);
v___x_791_ = lean_uint64_mod(v___x_790_, v___x_747_);
v___x_792_ = lean_uint64_mul(v_numerator_754_, v___x_791_);
v___x_793_ = lean_uint64_mod(v___x_792_, v___x_747_);
v___x_794_ = lean_uint64_mul(v_y0_722_, v___x_793_);
v___x_795_ = lean_uint64_sub(v___x_747_, v_a_727_);
v___x_796_ = lean_uint64_mul(v___x_795_, v___x_749_);
v___x_797_ = lean_uint64_mod(v___x_796_, v___x_747_);
v___x_798_ = lean_uint64_mul(v___x_797_, v___x_752_);
v_numerator_799_ = lean_uint64_mod(v___x_798_, v___x_747_);
v___x_800_ = lean_uint64_add(v_b_730_, v___x_747_);
v___x_801_ = lean_uint64_sub(v___x_800_, v_a_727_);
v___x_802_ = lean_uint64_sub(v___x_800_, v_c_733_);
v___x_803_ = lean_uint64_mul(v___x_801_, v___x_802_);
v___x_804_ = lean_uint64_mod(v___x_803_, v___x_747_);
v___x_805_ = lean_uint64_sub(v___x_800_, v_d_736_);
v___x_806_ = lean_uint64_mul(v___x_804_, v___x_805_);
v_denominator_807_ = lean_uint64_mod(v___x_806_, v___x_747_);
v_a_808_ = lean_uint64_mod(v_denominator_807_, v___x_747_);
v___x_809_ = lean_uint64_mul(v_a_808_, v_a_808_);
v_b_810_ = lean_uint64_mod(v___x_809_, v___x_747_);
v___x_811_ = lean_uint64_mul(v_b_810_, v_b_810_);
v_c_812_ = lean_uint64_mod(v___x_811_, v___x_747_);
v___x_813_ = lean_uint64_mul(v_c_812_, v_c_812_);
v_d_814_ = lean_uint64_mod(v___x_813_, v___x_747_);
v___x_815_ = lean_uint64_mul(v_d_814_, v_d_814_);
v_e_816_ = lean_uint64_mod(v___x_815_, v___x_747_);
v___x_817_ = lean_uint64_mul(v_e_816_, v_e_816_);
v_f_818_ = lean_uint64_mod(v___x_817_, v___x_747_);
v___x_819_ = lean_uint64_mul(v_f_818_, v_f_818_);
v_g_820_ = lean_uint64_mod(v___x_819_, v___x_747_);
v___x_821_ = lean_uint64_mul(v_g_820_, v_g_820_);
v_h_822_ = lean_uint64_mod(v___x_821_, v___x_747_);
v___x_823_ = lean_uint64_mul(v_a_808_, v_b_810_);
v___x_824_ = lean_uint64_mod(v___x_823_, v___x_747_);
v___x_825_ = lean_uint64_mul(v___x_824_, v_c_812_);
v___x_826_ = lean_uint64_mod(v___x_825_, v___x_747_);
v___x_827_ = lean_uint64_mul(v___x_826_, v_d_814_);
v___x_828_ = lean_uint64_mod(v___x_827_, v___x_747_);
v___x_829_ = lean_uint64_mul(v___x_828_, v_e_816_);
v___x_830_ = lean_uint64_mod(v___x_829_, v___x_747_);
v___x_831_ = lean_uint64_mul(v___x_830_, v_f_818_);
v___x_832_ = lean_uint64_mod(v___x_831_, v___x_747_);
v___x_833_ = lean_uint64_mul(v___x_832_, v_g_820_);
v___x_834_ = lean_uint64_mod(v___x_833_, v___x_747_);
v___x_835_ = lean_uint64_mul(v___x_834_, v_h_822_);
v___x_836_ = lean_uint64_mod(v___x_835_, v___x_747_);
v___x_837_ = lean_uint64_mul(v_numerator_799_, v___x_836_);
v___x_838_ = lean_uint64_mod(v___x_837_, v___x_747_);
v___x_839_ = lean_uint64_mul(v_y1_723_, v___x_838_);
v___x_840_ = lean_uint64_add(v___x_794_, v___x_839_);
v___x_841_ = lean_uint64_mul(v___x_795_, v___x_748_);
v___x_842_ = lean_uint64_mod(v___x_841_, v___x_747_);
v___x_843_ = lean_uint64_mul(v___x_842_, v___x_752_);
v_numerator_844_ = lean_uint64_mod(v___x_843_, v___x_747_);
v___x_845_ = lean_uint64_add(v_c_733_, v___x_747_);
v___x_846_ = lean_uint64_sub(v___x_845_, v_a_727_);
v___x_847_ = lean_uint64_sub(v___x_845_, v_b_730_);
v___x_848_ = lean_uint64_mul(v___x_846_, v___x_847_);
v___x_849_ = lean_uint64_mod(v___x_848_, v___x_747_);
v___x_850_ = lean_uint64_sub(v___x_845_, v_d_736_);
v___x_851_ = lean_uint64_mul(v___x_849_, v___x_850_);
v_denominator_852_ = lean_uint64_mod(v___x_851_, v___x_747_);
v_a_853_ = lean_uint64_mod(v_denominator_852_, v___x_747_);
v___x_854_ = lean_uint64_mul(v_a_853_, v_a_853_);
v_b_855_ = lean_uint64_mod(v___x_854_, v___x_747_);
v___x_856_ = lean_uint64_mul(v_b_855_, v_b_855_);
v_c_857_ = lean_uint64_mod(v___x_856_, v___x_747_);
v___x_858_ = lean_uint64_mul(v_c_857_, v_c_857_);
v_d_859_ = lean_uint64_mod(v___x_858_, v___x_747_);
v___x_860_ = lean_uint64_mul(v_d_859_, v_d_859_);
v_e_861_ = lean_uint64_mod(v___x_860_, v___x_747_);
v___x_862_ = lean_uint64_mul(v_e_861_, v_e_861_);
v_f_863_ = lean_uint64_mod(v___x_862_, v___x_747_);
v___x_864_ = lean_uint64_mul(v_f_863_, v_f_863_);
v_g_865_ = lean_uint64_mod(v___x_864_, v___x_747_);
v___x_866_ = lean_uint64_mul(v_g_865_, v_g_865_);
v_h_867_ = lean_uint64_mod(v___x_866_, v___x_747_);
v___x_868_ = lean_uint64_mul(v_a_853_, v_b_855_);
v___x_869_ = lean_uint64_mod(v___x_868_, v___x_747_);
v___x_870_ = lean_uint64_mul(v___x_869_, v_c_857_);
v___x_871_ = lean_uint64_mod(v___x_870_, v___x_747_);
v___x_872_ = lean_uint64_mul(v___x_871_, v_d_859_);
v___x_873_ = lean_uint64_mod(v___x_872_, v___x_747_);
v___x_874_ = lean_uint64_mul(v___x_873_, v_e_861_);
v___x_875_ = lean_uint64_mod(v___x_874_, v___x_747_);
v___x_876_ = lean_uint64_mul(v___x_875_, v_f_863_);
v___x_877_ = lean_uint64_mod(v___x_876_, v___x_747_);
v___x_878_ = lean_uint64_mul(v___x_877_, v_g_865_);
v___x_879_ = lean_uint64_mod(v___x_878_, v___x_747_);
v___x_880_ = lean_uint64_mul(v___x_879_, v_h_867_);
v___x_881_ = lean_uint64_mod(v___x_880_, v___x_747_);
v___x_882_ = lean_uint64_mul(v_numerator_844_, v___x_881_);
v___x_883_ = lean_uint64_mod(v___x_882_, v___x_747_);
v___x_884_ = lean_uint64_mul(v_y2_724_, v___x_883_);
v___x_885_ = lean_uint64_add(v___x_840_, v___x_884_);
v___x_886_ = lean_uint64_mul(v___x_842_, v___x_749_);
v_numerator_887_ = lean_uint64_mod(v___x_886_, v___x_747_);
v___x_888_ = lean_uint64_add(v_d_736_, v___x_747_);
v___x_889_ = lean_uint64_sub(v___x_888_, v_a_727_);
v___x_890_ = lean_uint64_sub(v___x_888_, v_b_730_);
v___x_891_ = lean_uint64_mul(v___x_889_, v___x_890_);
v___x_892_ = lean_uint64_mod(v___x_891_, v___x_747_);
v___x_893_ = lean_uint64_sub(v___x_888_, v_c_733_);
v___x_894_ = lean_uint64_mul(v___x_892_, v___x_893_);
v_denominator_895_ = lean_uint64_mod(v___x_894_, v___x_747_);
v_a_896_ = lean_uint64_mod(v_denominator_895_, v___x_747_);
v___x_897_ = lean_uint64_mul(v_a_896_, v_a_896_);
v_b_898_ = lean_uint64_mod(v___x_897_, v___x_747_);
v___x_899_ = lean_uint64_mul(v_b_898_, v_b_898_);
v_c_900_ = lean_uint64_mod(v___x_899_, v___x_747_);
v___x_901_ = lean_uint64_mul(v_c_900_, v_c_900_);
v_d_902_ = lean_uint64_mod(v___x_901_, v___x_747_);
v___x_903_ = lean_uint64_mul(v_d_902_, v_d_902_);
v_e_904_ = lean_uint64_mod(v___x_903_, v___x_747_);
v___x_905_ = lean_uint64_mul(v_e_904_, v_e_904_);
v_f_906_ = lean_uint64_mod(v___x_905_, v___x_747_);
v___x_907_ = lean_uint64_mul(v_f_906_, v_f_906_);
v_g_908_ = lean_uint64_mod(v___x_907_, v___x_747_);
v___x_909_ = lean_uint64_mul(v_g_908_, v_g_908_);
v_h_910_ = lean_uint64_mod(v___x_909_, v___x_747_);
v___x_911_ = lean_uint64_mul(v_a_896_, v_b_898_);
v___x_912_ = lean_uint64_mod(v___x_911_, v___x_747_);
v___x_913_ = lean_uint64_mul(v___x_912_, v_c_900_);
v___x_914_ = lean_uint64_mod(v___x_913_, v___x_747_);
v___x_915_ = lean_uint64_mul(v___x_914_, v_d_902_);
v___x_916_ = lean_uint64_mod(v___x_915_, v___x_747_);
v___x_917_ = lean_uint64_mul(v___x_916_, v_e_904_);
v___x_918_ = lean_uint64_mod(v___x_917_, v___x_747_);
v___x_919_ = lean_uint64_mul(v___x_918_, v_f_906_);
v___x_920_ = lean_uint64_mod(v___x_919_, v___x_747_);
v___x_921_ = lean_uint64_mul(v___x_920_, v_g_908_);
v___x_922_ = lean_uint64_mod(v___x_921_, v___x_747_);
v___x_923_ = lean_uint64_mul(v___x_922_, v_h_910_);
v___x_924_ = lean_uint64_mod(v___x_923_, v___x_747_);
v___x_925_ = lean_uint64_mul(v_numerator_887_, v___x_924_);
v___x_926_ = lean_uint64_mod(v___x_925_, v___x_747_);
v___x_927_ = lean_uint64_mul(v_y3_725_, v___x_926_);
v___x_928_ = lean_uint64_add(v___x_885_, v___x_927_);
v___x_929_ = lean_uint64_mod(v___x_928_, v___x_747_);
return v___x_929_;
}
else
{
uint64_t v___x_930_; 
v___x_930_ = 257ULL;
return v___x_930_;
}
}
else
{
uint64_t v___x_931_; 
v___x_931_ = 257ULL;
return v___x_931_;
}
}
else
{
uint64_t v___x_932_; 
v___x_932_ = 257ULL;
return v___x_932_;
}
}
else
{
uint64_t v___x_933_; 
v___x_933_ = 257ULL;
return v___x_933_;
}
}
}
}
v___jp_937_:
{
if (v___y_938_ == 0)
{
if (v___x_936_ == 0)
{
if (v___x_935_ == 0)
{
v___y_739_ = v___x_935_;
goto v___jp_738_;
}
else
{
v___y_739_ = v___x_934_;
goto v___jp_738_;
}
}
else
{
uint64_t v___x_939_; 
v___x_939_ = 257ULL;
return v___x_939_;
}
}
else
{
uint64_t v___x_940_; 
v___x_940_ = 257ULL;
return v___x_940_;
}
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_fieldReconstruct___boxed(lean_object* v_points_945_, lean_object* v_y0_946_, lean_object* v_y1_947_, lean_object* v_y2_948_, lean_object* v_y3_949_){
_start:
{
uint64_t v_points_boxed_950_; uint64_t v_y0_boxed_951_; uint64_t v_y1_boxed_952_; uint64_t v_y2_boxed_953_; uint64_t v_y3_boxed_954_; uint64_t v_res_955_; lean_object* v_r_956_; 
v_points_boxed_950_ = lean_unbox_uint64(v_points_945_);
lean_dec_ref(v_points_945_);
v_y0_boxed_951_ = lean_unbox_uint64(v_y0_946_);
lean_dec_ref(v_y0_946_);
v_y1_boxed_952_ = lean_unbox_uint64(v_y1_947_);
lean_dec_ref(v_y1_947_);
v_y2_boxed_953_ = lean_unbox_uint64(v_y2_948_);
lean_dec_ref(v_y2_948_);
v_y3_boxed_954_ = lean_unbox_uint64(v_y3_949_);
lean_dec_ref(v_y3_949_);
v_res_955_ = bastion_field_reconstruct(v_points_boxed_950_, v_y0_boxed_951_, v_y1_boxed_952_, v_y2_boxed_953_, v_y3_boxed_954_);
v_r_956_ = lean_box_uint64(v_res_955_);
return v_r_956_;
}
}
LEAN_EXPORT uint64_t bastion_unique_step(uint64_t v_state_957_, uint64_t v_equal_958_, uint64_t v_confirmed_959_){
_start:
{
uint64_t v___x_960_; uint8_t v___x_961_; 
v___x_960_ = 2ULL;
v___x_961_ = lean_uint64_dec_le(v___x_960_, v_state_957_);
if (v___x_961_ == 0)
{
uint64_t v___x_962_; uint8_t v___x_963_; 
v___x_962_ = 1ULL;
v___x_963_ = lean_uint64_dec_eq(v_confirmed_959_, v___x_962_);
if (v___x_963_ == 0)
{
return v_state_957_;
}
else
{
uint64_t v___x_964_; uint8_t v___x_965_; 
v___x_964_ = 0ULL;
v___x_965_ = lean_uint64_dec_eq(v_state_957_, v___x_964_);
if (v___x_965_ == 0)
{
uint8_t v___x_966_; 
v___x_966_ = lean_uint64_dec_eq(v_equal_958_, v___x_962_);
if (v___x_966_ == 0)
{
return v___x_960_;
}
else
{
return v___x_962_;
}
}
else
{
return v___x_962_;
}
}
}
else
{
return v___x_960_;
}
}
}
LEAN_EXPORT lean_object* l_Bastion_Runtime_uniqueStep___boxed(lean_object* v_state_967_, lean_object* v_equal_968_, lean_object* v_confirmed_969_){
_start:
{
uint64_t v_state_boxed_970_; uint64_t v_equal_boxed_971_; uint64_t v_confirmed_boxed_972_; uint64_t v_res_973_; lean_object* v_r_974_; 
v_state_boxed_970_ = lean_unbox_uint64(v_state_967_);
lean_dec_ref(v_state_967_);
v_equal_boxed_971_ = lean_unbox_uint64(v_equal_968_);
lean_dec_ref(v_equal_968_);
v_confirmed_boxed_972_ = lean_unbox_uint64(v_confirmed_969_);
lean_dec_ref(v_confirmed_969_);
v_res_973_ = bastion_unique_step(v_state_boxed_970_, v_equal_boxed_971_, v_confirmed_boxed_972_);
v_r_974_ = lean_box_uint64(v_res_973_);
return v_r_974_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Bastion_Runtime(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif

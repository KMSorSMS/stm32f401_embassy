    
    .extern  OS_CPU_EXCEPT_STK_BASE
    
    .extern  OS_TCB_HIGH_RDY
    .extern  OS_TCB_CUR
    .extern  OS_PRIO_CUR
    .extern  OS_PRIO_HIGH_RDY
    .extern  OS_IS_RUNNING
    
    .global  OSStartHighRdy                                     
    .global  OSIntCtxSw
    .global  OSCtxSw
    .global  PendSV_Handler
    .global  cli




.equ NVIC_INT_CTRL,     0xE000ED04                              
.equ NVIC_SYSPRI14,     0xE000ED22                              
.equ NVIC_PENDSV_PRI,   0xFF                                    
.equ NVIC_PENDSVSET,    0x10000000                              

    .text
    .align 2
    .thumb
    .syntax unified


.thumb_func
OSCtxSw:
OSIntCtxSw:
    LDR     R0, =NVIC_INT_CTRL                                  
    LDR     R1, =NVIC_PENDSVSET
    STR     R1, [R0]
    BX      LR


.thumb_func
PendSV_Handler:

    CPSID   I                                                   
    
    MRS     R0, PSP                                             
    STMFD   R0!, {{R4-R11, R14}}         

    LDR     R5, =OS_TCB_CUR
    LDR     R1, [R5]
    STR     R0, [R1]                                            

                                                                
    MOV     R4, LR                                              
    
    LDR     R0, =OS_PRIO_CUR
    LDR     R1, =OS_PRIO_HIGH_RDY
    LDRB    R2, [R1]
    STRB    R2, [R0]
    LDR     R1, =OS_TCB_HIGH_RDY
    LDR     R2, [R1]
    STR     R2, [R5]
    ORR     LR,  R4, #0x04                                      
    LDR     R0,  [R2]

    LDMFD   R0!, {{R4-R11, R14}}                                  
    MSR     PSP, R0                                             
    MOV     R2, #0                                              
    CPSIE   I
    BX      LR                                                  


.thumb_func
OSStartHighRdy:
    CPSID   I                                                   
    LDR     R0,=NVIC_SYSPRI14
    LDR     R1, =NVIC_PENDSV_PRI
    STRB    R1, [R0]
    MOVS    R0, #0                                              
    MSR     PSP, R0
    LDR     R0, =OS_CPU_EXCEPT_STK_BASE
    LDR     R1, [R0]
    MSR     MSP, R1
    LDR     R0, =OS_IS_RUNNING                                      
    MOVS    R1, #1
    STRB    R1, [R0]
    LDR     R0, =OS_PRIO_CUR
    LDR     R1, =OS_PRIO_HIGH_RDY
    LDRB    R2, [R1]
    STRB    R2, [R0]
    LDR     R0, =OS_TCB_CUR
    LDR     R1, =OS_TCB_HIGH_RDY
    LDR     R2, [R1]
    STR     R2, [R0]
    LDR     R0, [R2]                                            
    MSR     PSP, R0                                             
    MRS     R0, CONTROL
    ORR     R0, R0, #2
    MSR     CONTROL, R0
    ISB                                                         
    LDMFD    SP!, {{R4-R11, LR}}                                  
    LDMFD    SP!, {{R0-R3}}                                       
    LDMFD    SP!, {{R12, LR}}                                     
    LDMFD    SP!, {{R1, R2}}                                      
    CPSIE    I
    BX       R1

cli:
    MRS R0, xPSR
    STMFD sp!,{{R0}}
    MOV PC,LR

.end

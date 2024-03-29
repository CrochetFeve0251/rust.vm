enum Operation {
    Exit(i64),
    IntStore(i64),
    IntPrint(i64),
    IntTostring(i64),
    IntRandom(i64),
    JumpTo(i64),
    JumpZ(i64),
    JumpNz(i64),
    XorOp(i64),
    AddOp(i64),
    SubOp(i64),
    MulOp(i64),
    DivOp(i64),
    IncOp(i64),
    AndOp(i64),
    OrOp(i64),
    StringStore(i64),
    StringPrint(i64),
    StringConcat(i64),
    StringSystem(i64),
    StringToint(i64),
    CmpReg(i64),
    CmpImmediate(i64),
    CmpString(i64),
    IsString(i64),
    IsInteger(i64),
    NopOp(i64),
    RegStore(i64),
    Peek(i64),
    Poke(i64),
    Memcpy(i64),
    StackPush(i64),
    StackPop(i64),
    StackRet(i64),
    StackCall(i64),
    TrapOp(i64),
}
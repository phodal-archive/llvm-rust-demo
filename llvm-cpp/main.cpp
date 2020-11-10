#include <llvm/IR/Module.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Bitcode/BitcodeWriter.h>
#include <llvm/Support/raw_os_ostream.h>
#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/ExecutionEngine/GenericValue.h>

#include <string>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;
using namespace llvm;

int main() {
    LLVMContext context;
    IRBuilder<> builder(context);
    Module module("test", context);

    /*1.声明puts函数*/
    FunctionType *puts_type = FunctionType::get(builder.getInt32Ty(), {builder.getInt8PtrTy()}, true);
    Constant *puts_func = module.getOrInsertFunction("puts", puts_type);  // puts? printf?

    /*2.定义main函数*/
    FunctionType *main_type = FunctionType::get(builder.getInt32Ty(), false);
    Function *main_func = Function::Create(main_type, Function::ExternalLinkage, "main", &module);

    BasicBlock *main_body = BasicBlock::Create(context, "entry", main_func);
    builder.SetInsertPoint(main_body);

    CallInst *ret_value = builder.CreateCall(puts_func, builder.CreateGlobalStringPtr("Hello,world!!!"));
    builder.CreateRet(ret_value);

    /*3.output*/
    fstream fs("test.bc");  // lli test.bc
    raw_os_ostream out(fs);
    WriteBitcodeToFile(module, out);
    outs() << module;

    /*4.JIT*/
    InitializeNativeTarget();
    InitializeNativeTargetAsmPrinter();

    unique_ptr<Module> owner(&module);
    ExecutionEngine *engine = EngineBuilder(move(owner)).create();

    GenericValue gv = engine->runFunction(main_func, {});
    outs() << "Result: " << gv.IntVal;  // 获取函数调用结果

    return 0;
}

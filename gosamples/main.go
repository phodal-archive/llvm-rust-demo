package main

import (
	"fmt"
	"llvm.org/llvm/bindings/go/llvm"
)

func main() {
	// setup our builder and module
	builder := llvm.NewBuilder()
	mod := llvm.NewModule("my_module")

	// create our function prologue
	main := llvm.FunctionType(llvm.Int32Type(), []llvm.Type{}, false)
	llvm.AddFunction(mod, "main", main)
	block := llvm.AddBasicBlock(mod.NamedFunction("main"), "entry")
	builder.SetInsertPoint(block, block.FirstInstruction())

	// int a = 32
	a := builder.CreateAlloca(llvm.Int32Type(), "a")
	builder.CreateStore(llvm.ConstInt(llvm.Int32Type(), 32, false), a)

	// int b = 16
	b := builder.CreateAlloca(llvm.Int32Type(), "b")
	builder.CreateStore(llvm.ConstInt(llvm.Int32Type(), 16, false), b)

	// return a + b
	bVal := builder.CreateLoad(b, "b_val")
	aVal := builder.CreateLoad(a, "a_val")
	result := builder.CreateAdd(aVal, bVal, "ab_val")
	builder.CreateRet(result)

	// verify it's all good
	if ok := llvm.VerifyModule(mod, llvm.ReturnStatusAction); ok != nil {
		fmt.Println(ok.Error())
	}
	mod.Dump()

	// create our exe engine
	engine, err := llvm.NewExecutionEngine(mod)
	if err != nil {
		fmt.Println(err.Error())
	}

	// run the function!
	funcResult := engine.RunFunction(mod.NamedFunction("main"), []llvm.GenericValue{})
	fmt.Printf("%d\n", funcResult.Int(false))
}

package main

import (
	"bytes"
	"fmt"
	"runtime"

	"github.com/valyala/fasthttp"
)

var text = bytes.Repeat([]byte("X"), 8192)

func ServeText(ctx *fasthttp.RequestCtx) {
	//ctx.SetStatusCode(fasthttp.StatusOk)
	ctx.SetBody(text)
}

func ServeImage(ctx *fasthttp.RequestCtx) {
	fasthttp.ServeFile(ctx, "../../htdocs/sample.jpg")
}

func Router(ctx *fasthttp.RequestCtx) {
	switch string(ctx.Path()) {
	case "/sample.txt":
		ServeText(ctx)
	case "/sample.jpg":
		ServeImage(ctx)
	default:
		ctx.Error("Not Found", fasthttp.StatusNotFound)
	}
}

func main() {
	runtime.GOMAXPROCS(4)
	fmt.Println("Server started on http://127.0.0.1:8080")
	fasthttp.ListenAndServe(":8080", Router)
}

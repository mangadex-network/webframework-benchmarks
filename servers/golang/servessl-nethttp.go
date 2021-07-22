package main

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"os"
	"runtime"
	"strconv"
)

var text = bytes.Repeat([]byte("X"), 8192)

func ServeText(response http.ResponseWriter, request *http.Request) {
	// NOTE: Setting the size seems to greatly improve performance (https://github.com/golang/go/issues/41513)!
	response.Header().Set("Content-Length", "8192")
	//response.WriteHeader(http.StatusOK)
	response.Write(text)
}

// https://stackoverflow.com/questions/40747152/write-pipe-reading-into-http-response-in-golang
func ServeImage(response http.ResponseWriter, request *http.Request) {
	//http.ServeFile(response, request, "../../htdocs/sample.jpg")
	file, _ := os.Open("../../htdocs/sample.jpg")
	defer file.Close()
	info, _ := file.Stat()
	size := info.Size()
	// NOTE: Setting the size seems to greatly improve performance (https://github.com/golang/go/issues/41513)!
	response.Header().Set("Content-Length", strconv.FormatInt(size, 10))
	//response.Header().Set("Transfer-Encoding", "chunked")
	//response.WriteHeader(http.StatusOK)
	io.Copy(response, file)
}

func main() {
	runtime.GOMAXPROCS(4)
	fmt.Println("Server started on https://127.0.0.1:44300")
	http.HandleFunc("/sample.txt", ServeText)
	http.HandleFunc("/sample.jpg", ServeImage)
	http.ListenAndServeTLS(":44300", "../../localhost.crt", "../../localhost.key", nil)
}

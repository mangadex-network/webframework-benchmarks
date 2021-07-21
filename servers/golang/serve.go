package main

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"os"
)

var text = bytes.Repeat([]byte("X"), 8192)

func ServeText(response http.ResponseWriter, request *http.Request) {
	//response.WriteHeader(200)
	response.Write(text)
}

// https://github.com/golang/go/issues/41513
// https://stackoverflow.com/questions/40747152/write-pipe-reading-into-http-response-in-golang
func ServeImage(response http.ResponseWriter, request *http.Request) {
	file, _ := os.Open("../../htdocs/sample.jpg")
	defer file.Close()
	//response.WriteHeader(200)
	io.Copy(response, file)
}

func main() {
	fmt.Println("Server started on http://127.0.0.1:8080")
	http.HandleFunc("/sample.txt", ServeText)
	http.HandleFunc("/sample.jpg", ServeImage)
	http.ListenAndServe(":8080", nil)
}

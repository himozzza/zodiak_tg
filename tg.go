package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"regexp"
	"strings"
)

func main() {
	zodiakSign := "Рыбы"
	urlLink := "https://kirov-portal.ru/news/poslednie-novosti/znak-zodiaka-kotoromu-4-sentyabrya-stoit-doveritsya-sudbe-31855/"

	htmlDOM := getHTML(urlLink)
	zodiakForecast := zodiakForecast(zodiakSign, string(htmlDOM))
	fmt.Println(strings.TrimSpace(zodiakForecast))

}

func getHTML(urlLink string) string {
	resp, err := http.Get(urlLink)
	if err != nil {
		fmt.Println("Соединение с сайтом отсутствует.")
		os.Exit(0)
	}

	htmlDOM, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Не удалось открыть информацию с сайта.")
		os.Exit(0)
	}
	defer resp.Body.Close()

	return string(htmlDOM)
}

func zodiakForecast(zodiakSign, htmlDOM string) string {
	zodiakCompile := regexp.MustCompile(fmt.Sprintf("%s(.*\\s)(.*\\s)(.*)", zodiakSign))
	tagCompile := regexp.MustCompile("<(.*?)>")
	cleanCompile := regexp.MustCompile("&nbsp;")

	zodiakSplit := zodiakCompile.FindString(string(htmlDOM))
	tagRemove := tagCompile.ReplaceAllString(zodiakSplit, "\n")
	zodiakForecast := cleanCompile.ReplaceAllString(tagRemove, "")
	return zodiakForecast
	//
}

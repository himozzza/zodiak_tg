package main

import (
	"fmt"
	"io"
	"net/http"
	"regexp"
	"strings"
	"time"
)

func main() {
	var zodiakSigns = []string{
		"Овен",
		"Телец",
		"Близнецы",
		"Рак",
		"Лев",
		"Дева",
		"Весы",
		"Скорпион",
		"Стрелец",
		"Козерог",
		"Водолей",
		"Рыбы",
	}
	zodiacSign := zodiakSigns[7]
	links := getLinks()

	resp, _ := http.Get(links[0])
	defer resp.Body.Close()
	r, _ := io.ReadAll(resp.Body)
	fmt.Println(strings.SplitAfterN(string(r), zodiacSign, -1)[1])
	// form := fmt.Sprintf("%s(.*?</0-9>\n)", zodiacSign)

	// re, _ := regexp.Compile(form)
	// fmt.Println(re.FindString(string(r)))
}

func getLinks() []string {
	links := []string{}
	now := time.Now()
	var date string = fmt.Sprintf("%d+%s+%d", now.Year(), now.Month(), now.Day())
	googleUrl := fmt.Sprint("https://www.google.com/search?q=%D0%B3%D0%BE%D1%80%D0%BE%D1%81%D0%BA%D0%BE%D0%BF+")
	googleUrll := fmt.Sprint("&oq=%D0%B3%D0%BE%D1%80%D0%BE%D1%81%D0%BA%D0%BE%D0%BF+02+09+2022&aqs=chrome..69i57j0i546l5.17391j0j7&sourceid=chrome&ie=UTF-8")
	urlAddr := fmt.Sprintf("%s%s%s", googleUrl, date, googleUrll)

	resp, err := http.Get(urlAddr)
	if err != nil {
		fmt.Println(err)
	}
	defer resp.Body.Close()

	r, _ := io.ReadAll(resp.Body)
	re, _ := regexp.Compile("<a href=\"/url(.*?)\"")
	a := re.FindAllString(string(r), -1)
	re, _ = regexp.Compile("https://(.*?)&")
	for _, i := range a {
		linkFormat := re.FindString(i)
		if !strings.Contains(linkFormat, "google") {
			links = append(links, strings.SplitN(linkFormat, "&", -1)[0])
		}
	}
	return links
}

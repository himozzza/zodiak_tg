package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"regexp"
	"strings"
	"time"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func main() {
	// var zodiakSigns = []string{
	// 	"Овен",
	// 	"Телец",
	// 	"Близнецы",
	// 	"Рак",
	// 	"Лев",
	// 	"Дева",
	// 	"Весы",
	// 	"Скорпион",
	// 	"Стрелец",
	// 	"Козерог",
	// 	"Водолей",
	// 	"Рыбы",
	// }
	bot, err := tgbotapi.NewBotAPI("507849468:AAFpYe6fbKFFGU7qmbasK58PcqrQpRySqYE")
	if err != nil {
		log.Panic(err)
	}

	bot.Debug = true

	// log.Printf("Authorized on account %s", bot.Self.UserName)

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60

	updates := bot.GetUpdatesChan(u)

	for update := range updates {
		if update.Message != nil {
			// log.Printf("[%s] %s", update.Message.From.UserName, update.Message.Text)

			msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
			msg.ReplyToMessageID = update.Message.MessageID
			zodiacSign := update.Message.Text
			links := getLinks()
			b := calc(links, zodiacSign)
			fmt.Println(b)
			fmt.Println(update.Message.Text)
			bot.Send(tgbotapi.NewMessage(update.Message.Chat.ID, b))
		}
	}
}

func calc(links []string, zodiacSign string) string {
	u := 0
	var b string
	for _, i := range links {
		resp, err := http.Get(i)
		if err != nil {
			fmt.Println("Не удалось загрузить страницу.")
		}
		defer resp.Body.Close()
		r, _ := io.ReadAll(resp.Body)
		form := fmt.Sprintf("%s[\\w\\d</>\\s]*(.*)", zodiacSign)
		re, _ := regexp.Compile(form)
		a := re.FindString(string(r))
		if strings.Contains(a, zodiacSign) {
			u++
			re := regexp.MustCompile(`[a-z/<>0-9;&]+`)
			b := re.ReplaceAllString(a, " ")
			if len(b) < 146 {
				continue
			}
			return b
			fmt.Printf("\n\n\n\n\n\n\n\n\n")
			if u == 5 {
				break
			}
		}
	}
	return b
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

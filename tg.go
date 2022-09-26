package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"regexp"
	"strings"
	"sync"
	"time"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func main() {
	bot, err := tgbotapi.NewBotAPI("TOKEN")
	if err != nil {
		log.Panic(err)
	}
	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60

	var zodiakSigns = map[string]string{
		"Овен":     "aries",
		"Телец":    "taurus",
		"Близнецы": "gemini",
		"Рак":      "cancer",
		"Лев":      "leo",
		"Дева":     "virgo",
		"Весы":     "libra",
		"Скорпион": "scorpio",
		"Стрелец":  "sagittarius",
		"Козерог":  "capricorn",
		"Водолей":  "aquarius",
		"Рыбы":     "pisces",
	}
	forecasts := make(map[string]string)
	updates := bot.GetUpdatesChan(u)
	c := make(chan string)
	var wg sync.WaitGroup
	for update := range updates {
		if update.Message != nil {
			// log.Printf("[%s] %s", update.Message.From.UserName, update.Message.Text)

			msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
			msg.ReplyToMessageID = update.Message.MessageID
			zodiakSign := update.Message.Text

			for n, i := range zodiakSigns {
				wg.Add(1)
				go getForecast(n, i, c, &wg)
				time.Sleep(10 * time.Millisecond)
				forecasts[n] = <-c
			}
			wg.Wait()
			if strings.Contains(forecasts[zodiakSign], "Фатальная ошибка!") {
				bot.Send(tgbotapi.NewMessage(update.Message.Chat.ID, "Не удалось подключиться к серверу.\nПопробуйте позже."))
			} else {
				bot.Send(tgbotapi.NewMessage(update.Message.Chat.ID, forecasts[zodiakSign]))
			}
		}
	}
}

func getForecast(n, i string, c chan string, wg *sync.WaitGroup) {
	resp, err := http.Get(fmt.Sprintf("https://horo.mail.ru/prediction/%s/today/", i))
	if err != nil {
		fmt.Println("Не удалось подключиться к серверу.", err)
		c <- "Фатальная ошибка!"
		wg.Done()
	}

	html, _ := io.ReadAll(resp.Body)
	re := regexp.MustCompile(`<p>(.*?)</p>`)
	reply := re.FindAllString(string(html), -1)
	re = regexp.MustCompile(`(<p>)|</p>`)
	forecast := re.ReplaceAllString(fmt.Sprintf("%s\n\n%s\n%s", n, reply[0], reply[1]), "")
	re = regexp.MustCompile(`&(.*);`)
	forecast = re.ReplaceAllString(forecast, " ")
	c <- forecast
	wg.Done()
}

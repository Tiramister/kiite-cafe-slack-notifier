# kiite-cafe-slack-notifier

## 準備

Slack の通知先への Webhook URL を環境変数に設定する。

```shell
$ export WEBHOOK_URL=https://hooks.slack.com/services/...
```

通知対象とする曲は、`videos.txt` に行区切りで指定する（ゆくゆくは自動取得にしたい）。

```shell
$ read LIST_URL; curl "https://cafe.kiite.jp/api/playlists/contents/detail?list_id=$(basename $LIST_URL)" | jq -r '.songs |  .[].video_id' >> videos.txt
# 入力待ちになるので、Kiite プレイリストの URL を入力する
https://kiite.jp/playlist/XXXXXXXX
```


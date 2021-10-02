# poll

> Cloudflare worker for embedding polls anywhere.

| Pineapple on pizza?                                                                                                 |                                                               |
| ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| [👍 Yes](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes&redirect=https://github.com/vberlier/poll%23poll) | ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=yes) |
| [👎 No](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=no&redirect=https://github.com/vberlier/poll%23poll)   | ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=no)  |

**Features**

- No sign up, just paste urls in your html/markdown
- Unlimited polls and unlimited options per poll
- GDPR compliant, no tracking, the service doesn't store any personal information

## Getting started

To create a poll you first need to come up with a unique namespaced identifier.

```
vberlier.pineapple_pizza
```

Then use the available endpoints to compose your poll however you like:

- `https://poll.fizzy.wtf/show?<poll>=<option>`

  The `show` endpoint is meant to be used as an image. It responds with a horizontal bar that's filled according to the number of votes for the specified option.

  ```
  https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=yes
  ```

- `https://poll.fizzy.wtf/vote?<poll>=<option>`

  The `vote` endpoint is meant to be used with a clickable link to let the user vote for the specified option.

  ```
  https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=no
  ```

  By default, after clicking the link, the endpoint will bring the user back to the previous page with `history.back()`. However this can cause a slight flicker when voting so you can specify an explicit `redirect` parameter to make the endpoint return a 302 to the url of your choice.

  ```
  https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes&redirect=https://github.com/vberlier/poll
  ```

## Contributing

Contributions are welcome. Make sure to first open an issue discussing the problem or the new feature before creating a pull request.

---

License - [MIT](https://github.com/vberlier/poll/blob/main/LICENSE)

# poll.fizzy.wtf

> Cloudflare worker for embedding polls anywhere.

**🍕 Pineapple on pizza? 🍍**

|                                                                                                                     |                                                               |                                                                |
| ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------: | -------------------------------------------------------------: |
| [Yes 👍](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes&redirect=https://github.com/vberlier/poll%23poll) | ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=yes) | ![](https://poll.fizzy.wtf/count?vberlier.pineapple_pizza=yes) |
| [No 👎](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=no&redirect=https://github.com/vberlier/poll%23poll)   |  ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=no) |  ![](https://poll.fizzy.wtf/count?vberlier.pineapple_pizza=no) |
|                                                                                                                     |                                                         Total |     ![](https://poll.fizzy.wtf/count?vberlier.pineapple_pizza) |

**Features**

- Unlimited polls and unlimited options per poll
- No setup required, polls and all their options are created on-the-fly
- No sign up, just paste urls in your html/markdown
- GDPR compliant, no tracking, the worker doesn't store any personal information

## Getting started

The worker provides a voting endpoint that you can use as a clickable link. When someone clicks the voting link, the worker increments a counter associated with the selected option and redirects them to the previous page. If the user clicks any of the options again, nothing will happen because the worker will remember that they already voted on this poll.

- **Voting endpoint**

  ```
  https://poll.fizzy.wtf/vote?<poll>=<option>
  ```

In addition to the voting endpoint, the worker can render svg widgets for showing the results of the poll. The widget endpoints are meant to be used directly as images.

- **Widget endpoints**

  ```
  https://poll.fizzy.wtf/show?<poll>=<option>
  https://poll.fizzy.wtf/count?<poll>
  https://poll.fizzy.wtf/count?<poll>=<option>
  ```

That's it! You can now create dynamic polls anywhere. Just add a voting link for each option and use some of the available widgets to show the results.

## Creating a poll step by step

The first thing to do is to come up with a unique scoped identifier for your poll. It needs to contain a `.` (dot character) to separate the scope from the name of the poll.

```
vberlier.pineapple_pizza
```

Then add voting links for each option. You can display the options however you want as long as the user can click on the voting links.

```md
Pineapple on pizza?

- [Yes](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes)

- [No](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=no)
```

Now to display the results we're going to use the `show` endpoint to render a horizontal bar filled according to the number of votes for each option.

```md
Pineapple on pizza?

- [Yes](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes)

  ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=yes)

- [No](https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=no)

  ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=no)
```

We're done! Feel free to be creative when it comes to layout and formatting. For example if you wanted to keep the results hidden by default you could wrap the horizontal bar in a `<details>` element.

## Clean redirections

By default, after clicking the voting link, the worker will bring the user back to the previous page by using a script that calls `history.back()`. However this can cause a slight flicker when voting so you can specify an explicit `redirect` parameter to make the endpoint return a 302 to the url of your choice.

```
https://poll.fizzy.wtf/vote?vberlier.pineapple_pizza=yes&redirect=https://github.com/vberlier/poll
```

## Available widgets

- **Horizontal bar**

  ![](https://poll.fizzy.wtf/show?vberlier.pineapple_pizza=yes)

  ```
  https://poll.fizzy.wtf/show?<poll>=<option>
  ```

- **Vote count**

  ![](https://poll.fizzy.wtf/count?vberlier.pineapple_pizza=yes)

  ```
  https://poll.fizzy.wtf/count?<poll>=<option>
  ```

- **Total vote count**

  ![](https://poll.fizzy.wtf/count?vberlier.pineapple_pizza)

  ```
  https://poll.fizzy.wtf/count?<poll>
  ```

## Contributing

Contributions are welcome. Make sure to first open an issue discussing the problem or the new feature before creating a pull request.

---

License - [MIT](https://github.com/vberlier/poll/blob/main/LICENSE)

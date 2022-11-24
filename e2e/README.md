## End-to-end test environment
This folder holds set of useful tools for testing in a e2e manner. 

### Run e2e script

```shell
$ docker-compose up --build
```

If you run the command above:

* test gaia chain starts.
  * if you want to customize the chain yourself, modify `e2e/scripts/init_chain.sh`
* background process sending `MsgSend` transaction for each second.
  * the process name is called `flood`, which floods a transaction for each second.
* postgres database
* database viewer called adminer
  * please visit `localhost:8082` in your browser.
* indexer start to crawl all the blocks, transactions and events from the test chain, and then it stores them in the dataase.
* api server start to listen requests.

node packages/moriarty-beta/dist/cli.js check packages/moriarty-beta/examples/local/repay/repayment.mori --json
node packages/moriarty-beta/dist/cli.js simulate packages/moriarty-beta/examples/local/repay/repayment.mori \
  --action repay_loan --scenario packages/moriarty-beta/examples/local/repay/scenario.json
node packages/moriarty-beta/dist/cli.js test packages/moriarty-beta/examples/local/repay

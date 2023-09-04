# Decentralized Price Checker in Rust


# Introduction

This project is a simple command-line-tool (CLI) built in Rust that allows users to compare swap prices from multiple mock exchanges.

The goal of the project is to find the best price and demonstrate a simple mechanism that could prevent bad actors from manipulating prices.

The project is a basic representation of how decentralized oracles work, providing price feeds to exchanges.

# Project's Objectives

* Compare swap prices from multiple mock exchanges

* Detect any anomalies in the price feeds from the exchanges

* Present the best swap rate for the user.

N.B. For the project I'll only focus on two tokens, USDC and USDT.

# Solution

* Each mock exchange will have its own liquidity pool of USDC, USDT tokens

* The swap rate will be determined by the relative liquidity of the two tokens

* Swaps will affect liquidity of the tokens in the selected exchange

* The tool will display not only the best rate but also the available liquidity to the user



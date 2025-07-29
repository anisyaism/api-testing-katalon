# API Testing using Katalon

## 🔍 What is Chaining API?
Chaining API is extracting data from one API response then use them for request in another API.

## 💡 What is done in this branch
In this branch we want to chaining JSON format. Assume that we already have REST API. For example in this project we have **Create User** API and after we send a POST request, we get response body with new user "id" is 12. After we successfully create a new user and get the "id", we can verify the data has been store to database or not by checking in **[GET] list User by id** using this chaining API technique. That's what is meant by JSON chaining.

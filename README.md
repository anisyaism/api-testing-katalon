# API Testing using Katalon

## 🔍 What is Chaining API?
Chaining API is extracting data from one API response then use them for request in another API.

## 💡 What is done in this branch
In this branch we want to chaining XML format, as we know for using XML we have to used SOAP API because SOAP API relies on XML for defining the structure of the messages. Assume that we already have SOAP API. For example in this project we have _Country ISO Code_ Service and after we send a request, we get response is "ID" for Indonesia ISO Code. After we get the ISO Code we can use it in another API for example we can use this ISO code for requesting _Capital City_ with ISO Code is "ID". After that we can verifying the response of _Capital City_ service it's matched with what expected or not. That's what is meant by XML chaining.


 




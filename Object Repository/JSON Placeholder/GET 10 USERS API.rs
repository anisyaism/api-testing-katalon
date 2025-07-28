<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET 10 USERS API</name>
   <tag></tag>
   <elementGuidId>60344f74-7ffc-4eb1-8d1f-c27b6cb3b182</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BASE_URL_JSONPlaceholder}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>d7ff1462-82a7-4da7-aafd-656d4b5dfd5d</id>
      <name>Validate JSON Schema</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\Lenovo\Desktop\GET 100 Users API JSON SCHEMA.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL_JSONPlaceholder</defaultValue>
      <description></description>
      <id>1f296090-5df0-455e-bf8e-0c8864932dea</id>
      <masked>false</masked>
      <name>BASE_URL_JSONPlaceholder</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

// Get Current Request 
RequestObject request = WSResponseManager.getInstance().getCurrentRequest() 

// Get Current Response
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


// Verify Response Body Contains String
assertThat(response.getResponseText()).contains('Leanne Graham')

// Verify Status Code is equals to 200
WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


def variables = request.getVariables()
def variable = variables.get('yourVariableName')

// Verify Base URl / Global Variable
def expGlobalVariable = 'https://jsonplaceholder.typicode.com'
def actGlobalVariable = GlobalVariable.BASE_URL_JSONPlaceholder

if(expGlobalVariable == actGlobalVariable ) {
	println 'Same as expected'
}else {
	println 'FAILED'
}


// Verify Response Body JSON Value Check
WS.verifyElementPropertyValue(response, 'address[0].street', 'Kulas Light')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

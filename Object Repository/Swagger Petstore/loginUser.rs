<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>loginUser</name>
   <tag></tag>
   <elementGuidId>fd2127f6-8374-4af1-ae8b-9ee634573ba5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>be0445aa-28ff-43e7-a6b8-0ad82248a382</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0b0ec024-fc10-4ab1-bca2-ae55a644ee28</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path>/user/login</path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/login?username=${USERNAME}&amp;password=${PASSWORD}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.USERNAME</defaultValue>
      <description></description>
      <id>5f745a72-914a-49d9-be75-331172019e99</id>
      <masked>false</masked>
      <name>USERNAME</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PASSWORD</defaultValue>
      <description></description>
      <id>c8680e69-9a34-4ccd-be86-d4a661dccabb</id>
      <masked>false</masked>
      <name>PASSWORD</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def userName = WS.getElementText(response, 'username')
println(&quot;USERNAME = &quot; + userName)
def pass = WS.getElementText(response, 'password')
println(&quot;PASS = &quot; + pass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

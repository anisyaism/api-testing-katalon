<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getUserByName</name>
   <tag></tag>
   <elementGuidId>52863153-c428-4421-862c-b4c7850ce602</elementGuidId>
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
      <webElementGuid>a3340af9-3918-4a04-9ad4-fa41398cc635</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9266d17b-9ae7-47f1-ac53-6432161ca263</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path>/user/{username}</path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/anisyatesting</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'anisyatesting'</defaultValue>
      <description></description>
      <id>65ec6b8d-74b6-4392-b964-116163fc43cf</id>
      <masked>false</masked>
      <name>username2</name>
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

GlobalVariable.USERNAME = WS.getElementPropertyValue(response, 'username')

println(&quot;USERNAME = &quot; + GlobalVariable.USERNAME )

GlobalVariable.PASSWORD = WS.getElementPropertyValue(response, 'password')

println(&quot;PASS = &quot; + GlobalVariable.PASSWORD )

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

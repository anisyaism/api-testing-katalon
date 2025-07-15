<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update (1)</name>
   <tag></tag>
   <elementGuidId>1ad9f022-48d0-4b0b-abe1-8e74f444c96f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus HIHI\&quot;,\n    \&quot;job\&quot;: \&quot;zion resident\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>c5173176-047e-41a7-82ef-837779d6a154</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>reqres-free-v1</value>
      <webElementGuid>eb72c1a8-1c1a-4d0f-b28c-dea1a2a3604b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${BASE_URL_REQRES}/api/users/2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL_REQRES</defaultValue>
      <description></description>
      <id>f8b590b5-31dd-4418-93ce-eb139dbc708f</id>
      <masked>false</masked>
      <name>BASE_URL_REQRES</name>
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

def getResponse = WS.getElementPropertyValue(response, 'name')
def getElementText = WS.getElementText(response, 'name')
println (&quot;THE UPDATE NAME = &quot; + getElementText)
def getElementText2 = WS.getElementText(response, 'job')
println (&quot;THE UPDATE JOB = &quot; + getElementText2)
def getElementText3 = WS.getElementText(response, 'updatedAt')
println (&quot;THE UPDATE USER TIME AT = &quot; + getElementText3)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>other-ins-feedback_CPR</name>
   <tag></tag>
   <elementGuidId>5e5ea414-25c7-4e6d-9e40-7625099c2da7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;FeedbackOrderInsCarWarrantyRq\&quot;: {\n        \&quot;UserId\&quot;: \&quot;${id}\&quot;,\n        \&quot;OrderDate\&quot;: \&quot;${order_date}\&quot;,\n        \&quot;Source\&quot;: \&quot;${source}\&quot;,\n        \&quot;Data\&quot;: {\n            \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n            \&quot;NoPolis\&quot;: \&quot;${nopolis}\&quot;\n        }\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/otherfeedbackorderinscarwarranty</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ewNZQHrDyoayIexbE4sxzeujJQGJ'</defaultValue>
      <description></description>
      <id>4946716f-c100-43a8-b908-ff25b6cd1a59</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'CPR 03'</defaultValue>
      <description></description>
      <id>60393f34-7691-4354-9ba4-496489a42455</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'2021-12-31'</defaultValue>
      <description></description>
      <id>3c63e9a0-3eea-4f1e-b011-e639cf55766e</id>
      <masked>false</masked>
      <name>order_date</name>
   </variables>
   <variables>
      <defaultValue>'PGI'</defaultValue>
      <description></description>
      <id>fef18861-f049-40a9-ae68-d4a3972e2cf3</id>
      <masked>false</masked>
      <name>source</name>
   </variables>
   <variables>
      <defaultValue>'1161008171001'</defaultValue>
      <description></description>
      <id>0ea0bb51-575d-46ab-8377-9bcda0dd8c48</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'NoPolis-20211231-1'</defaultValue>
      <description></description>
      <id>2ad3f49d-a24a-45a6-809b-32f911a3b02f</id>
      <masked>false</masked>
      <name>nopolis</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

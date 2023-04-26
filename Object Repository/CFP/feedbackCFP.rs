<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>feedbackCFP</name>
   <tag></tag>
   <elementGuidId>332f27da-beee-44de-b615-fc6539868852</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;RequestParameter\&quot;: {\n        \&quot;NoKontrak\&quot;: \&quot;${nokontrak}\&quot;,\n        \&quot;NoRangka\&quot;: \&quot;${norangka}\&quot;,\n        \&quot;NoIMEI\&quot;: \&quot;${noimei}\&quot;,\n        \&quot;NoGSM\&quot;: \&quot;${nogsm}\&quot;,\n        \&quot;UserName\&quot;: \&quot;${username}\&quot;,\n        \&quot;StatusCode\&quot;: \&quot;3\&quot;\n    },\n    \&quot;Source\&quot;: \&quot;FOXLOGGER\&quot;\n}&quot;,
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
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/otherfeedbackorderstscfp</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>088b346d-2bfa-4f72-9e5a-7b40925abc4f</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'234JIIOE'</defaultValue>
      <description></description>
      <id>c33f5817-1029-4bbf-b755-f7ad15d8d7ba</id>
      <masked>false</masked>
      <name>norangka</name>
   </variables>
   <variables>
      <defaultValue>'9430710308001'</defaultValue>
      <description></description>
      <id>46eef72b-9a58-48b5-a50c-bbeb17424848</id>
      <masked>false</masked>
      <name>nokontrak</name>
   </variables>
   <variables>
      <defaultValue>'IMEI-2rev'</defaultValue>
      <description></description>
      <id>d7b266ab-9b54-4d05-ad5d-0915f38df0af</id>
      <masked>false</masked>
      <name>noimei</name>
   </variables>
   <variables>
      <defaultValue>'990348999'</defaultValue>
      <description></description>
      <id>9445e7f4-8cc3-4f8e-a508-d361075c29de</id>
      <masked>false</masked>
      <name>nogsm</name>
   </variables>
   <variables>
      <defaultValue>'VHETEST'</defaultValue>
      <description></description>
      <id>d3ec9304-946d-49b3-9b02-f1959be1672b</id>
      <masked>false</masked>
      <name>username</name>
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

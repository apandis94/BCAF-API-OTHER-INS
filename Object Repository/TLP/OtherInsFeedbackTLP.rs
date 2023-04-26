<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OtherInsFeedbackTLP</name>
   <tag></tag>
   <elementGuidId>8651124c-85c3-48dc-bd75-13a960c6c032</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;SubmitPolisAsuransiTLCRequest\&quot;: {\n        \&quot;UserID\&quot;: \&quot;${id}\&quot;,\n        \&quot;MaskapaiID\&quot;: \&quot;${maskapai_id}\&quot;,\n        \&quot;OrderDate\&quot;: \&quot;${orderdate}\&quot;,\n        \&quot;Source\&quot;: \&quot;${source}\&quot;,\n        \&quot;Details\&quot;: [\n            {\n                \&quot;NoReferensi\&quot;: \&quot;${no_reff}\&quot;,\n                \&quot;PolisNo\&quot;: \&quot;${no_polis}\&quot;\n            }\n        ]\n    }\n}&quot;,
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
      <webElementGuid>b1e34c56-489e-405d-b93a-91d6d21c1b09</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>5aa47c09-c94a-466a-bd52-0cbf99fe450c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/FeedbackPolisInsOrderTLP</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'owRpbUWJG0ZOKQPfLUnfb1a5b0mq'</defaultValue>
      <description></description>
      <id>11a7ee14-61eb-435e-a609-3e7d86c4649c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'TLP'</defaultValue>
      <description></description>
      <id>da787563-d25b-4de9-ad34-17a7cc8b61fa</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'TLP'</defaultValue>
      <description></description>
      <id>ebcd8669-c742-4aa6-9f4c-8971bd02b768</id>
      <masked>false</masked>
      <name>maskapai_id</name>
   </variables>
   <variables>
      <defaultValue>'AXA'</defaultValue>
      <description></description>
      <id>ab08d62a-7e3f-4c7e-9fe2-c1c89d689cfb</id>
      <masked>false</masked>
      <name>source</name>
   </variables>
   <variables>
      <defaultValue>'2022-04-19'</defaultValue>
      <description></description>
      <id>c0f5417d-81b1-49e2-82b5-0cdd85bbd3b1</id>
      <masked>false</masked>
      <name>orderdate</name>
   </variables>
   <variables>
      <defaultValue>'9430010123003'</defaultValue>
      <description></description>
      <id>f56a2343-92ab-42ef-9ca0-3dd222dad783</id>
      <masked>false</masked>
      <name>no_reff</name>
   </variables>
   <variables>
      <defaultValue>'No1-APITLP-20220422'</defaultValue>
      <description></description>
      <id>aac96f3a-08ec-45b7-b7a5-2122b904af50</id>
      <masked>false</masked>
      <name>no_polis</name>
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

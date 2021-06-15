<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST transaction not valid</name>
   <tag></tag>
   <elementGuidId>6c316d20-a361-40e2-beac-595ccb13088b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;GroupHeader\&quot;: {\n        \&quot;InitiatingParty\&quot;: {\n            \&quot;Name\&quot;: \&quot;payerfirstName payermiddleName payerlastname\&quot;,\n            \&quot;Identification\&quot;: {\n                \&quot;Identification\&quot;: \&quot;\&quot;,\n                \&quot;Other\&quot;: {\n                    \&quot;Identification\&quot;: \&quot;\&quot;,\n                    \&quot;SchemeName\&quot;: {\n                        \&quot;Proprietary\&quot;: \&quot;\&quot;\n                    },\n                    \&quot;PrivateIdentification\&quot;: {\n                        \&quot;DateAndPlaceOfBirth\&quot;: {\n                            \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                            \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                            \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                            \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                        }\n                    },\n                    \&quot;ContactDetails\&quot;: {\n                        \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                    }\n                },\n                \&quot;SchemeName\&quot;: {\n                    \&quot;Proprietary\&quot;: \&quot;\&quot;\n                },\n                \&quot;PrivateIdentification\&quot;: {\n                    \&quot;DateAndPlaceOfBirth\&quot;: {\n                        \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                        \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                        \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                        \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                    }\n                },\n                \&quot;ContactDetails\&quot;: {\n                    \&quot;MobileNumber\&quot;: \&quot;+277-23748020\&quot;\n                }\n            }\n        }\n    },\n    \&quot;PaymentInformation\&quot;: {\n        \&quot;PaymentInformationIdentification\&quot;: \&quot;ABC123\&quot;,\n        \&quot;CreditTransferTransactionInformation\&quot;: {\n            \&quot;PaymentIdentification\&quot;: {\n                \&quot;EndToEndIdentification\&quot;: \&quot;asdf1234\&quot;\n            },\n            \&quot;CreditorAccount\&quot;: {\n                \&quot;Identification\&quot;: {\n                    \&quot;Identification\&quot;: \&quot;\&quot;,\n                    \&quot;Other\&quot;: {\n                        \&quot;Identification\&quot;: \&quot;+27723748019\&quot;,\n                        \&quot;SchemeName\&quot;: {\n                            \&quot;Proprietary\&quot;: \&quot;MSISDN\&quot;\n                        },\n                        \&quot;PrivateIdentification\&quot;: {\n                            \&quot;DateAndPlaceOfBirth\&quot;: {\n                                \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                                \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                                \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                                \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                            }\n                        },\n                        \&quot;ContactDetails\&quot;: {\n                            \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                        }\n                    },\n                    \&quot;SchemeName\&quot;: {\n                        \&quot;Proprietary\&quot;: \&quot;\&quot;\n                    },\n                    \&quot;PrivateIdentification\&quot;: {\n                        \&quot;DateAndPlaceOfBirth\&quot;: {\n                            \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                            \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                            \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                            \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                        }\n                    },\n                    \&quot;ContactDetails\&quot;: {\n                        \&quot;MobileNumber\&quot;: \&quot;+277-23748019\&quot;\n                    }\n                },\n                \&quot;Proxy\&quot;: \&quot;\&quot;,\n                \&quot;Name\&quot;: \&quot;\&quot;\n            },\n            \&quot;CreditorAgent\&quot;: {\n                \&quot;FinancialInstitutionIdentification\&quot;: {\n                    \&quot;ClearingSystemMemberIdentification\&quot;: {\n                        \&quot;MemberIdentification\&quot;: \&quot;bank1\&quot;\n                    }\n                }\n            },\n            \&quot;Creditor\&quot;: {\n                \&quot;Name\&quot;: \&quot;payeefirstName payeemiddleName payeelastname\&quot;,\n                \&quot;Identification\&quot;: {\n                    \&quot;Identification\&quot;: \&quot;\&quot;,\n                    \&quot;Other\&quot;: {\n                        \&quot;Identification\&quot;: \&quot;\&quot;,\n                        \&quot;SchemeName\&quot;: {\n                            \&quot;Proprietary\&quot;: \&quot;\&quot;\n                        },\n                        \&quot;PrivateIdentification\&quot;: {\n                            \&quot;DateAndPlaceOfBirth\&quot;: {\n                                \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                                \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                                \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                                \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                            }\n                        },\n                        \&quot;ContactDetails\&quot;: {\n                            \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                        }\n                    },\n                    \&quot;SchemeName\&quot;: {\n                        \&quot;Proprietary\&quot;: \&quot;\&quot;\n                    },\n                    \&quot;PrivateIdentification\&quot;: {\n                        \&quot;DateAndPlaceOfBirth\&quot;: {\n                            \&quot;Birthdate\&quot;: \&quot;2021-05-28\&quot;,\n                            \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                            \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                            \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                        }\n                    },\n                    \&quot;ContactDetails\&quot;: {\n                        \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                    }\n                }\n            },\n            \&quot;Amount\&quot;: {\n                \&quot;InstructedAmount\&quot;: {},\n                \&quot;EquivalentAmount\&quot;: {\n                    \&quot;CurrencyOfTransfer\&quot;: \&quot;USD\&quot;,\n                    \&quot;Amount\&quot;: 123.45\n                }\n            },\n            \&quot;SupplementaryData\&quot;: {\n                \&quot;fees.currency\&quot;: \&quot;USD\&quot;,\n                \&quot;fees.amount\&quot;: 12.34\n            },\n            \&quot;PaymentTypeInformation\&quot;: {\n                \&quot;CategoryPurpose\&quot;: {\n                    \&quot;Proprietary\&quot;: \&quot;DEPOSIT\&quot;\n                }\n            },\n            \&quot;RegulatoryReporting\&quot;: {\n                \&quot;Details\&quot;: {\n                    \&quot;Code\&quot;: \&quot;string\&quot;\n                }\n            },\n            \&quot;RemittanceInformation\&quot;: {\n                \&quot;Structured\&quot;: {\n                    \&quot;AdditionalRemittanceInformation\&quot;: \&quot;string\&quot;\n                }\n            }\n        },\n        \&quot;DebtorAccount\&quot;: {\n            \&quot;Identification\&quot;: {\n                \&quot;Identification\&quot;: \&quot;\&quot;,\n                \&quot;Other\&quot;: {\n                    \&quot;Identification\&quot;: \&quot;+27723748020\&quot;,\n                    \&quot;SchemeName\&quot;: {\n                        \&quot;Proprietary\&quot;: \&quot;MSISDN\&quot;\n                    },\n                    \&quot;PrivateIdentification\&quot;: {\n                        \&quot;DateAndPlaceOfBirth\&quot;: {\n                            \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                            \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                            \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                            \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                        }\n                    },\n                    \&quot;ContactDetails\&quot;: {\n                        \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                    }\n                },\n                \&quot;SchemeName\&quot;: {\n                    \&quot;Proprietary\&quot;: \&quot;\&quot;\n                },\n                \&quot;PrivateIdentification\&quot;: {\n                    \&quot;DateAndPlaceOfBirth\&quot;: {\n                        \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                        \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                        \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                        \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                    }\n                },\n                \&quot;ContactDetails\&quot;: {\n                    \&quot;MobileNumber\&quot;: \&quot;\&quot;\n                }\n            },\n            \&quot;Proxy\&quot;: \&quot;string\&quot;,\n            \&quot;Name\&quot;: \&quot;\&quot;\n        },\n        \&quot;DebtorAgent\&quot;: {\n            \&quot;FinancialInstitutionIdentification\&quot;: {\n                \&quot;ClearingSystemMemberIdentification\&quot;: {\n                    \&quot;MemberIdentification\&quot;: \&quot;string\&quot;\n                }\n            }\n        },\n        \&quot;Debtor\&quot;: {\n            \&quot;Name\&quot;: \&quot;payerfirstName payermiddleName payerlastname\&quot;,\n            \&quot;Identification\&quot;: {\n                \&quot;Identification\&quot;: \&quot;\&quot;,\n                \&quot;Other\&quot;: {\n                    \&quot;Identification\&quot;: \&quot;\&quot;,\n                    \&quot;SchemeName\&quot;: {\n                        \&quot;Proprietary\&quot;: \&quot;\&quot;\n                    },\n                    \&quot;PrivateIdentification\&quot;: {\n                        \&quot;DateAndPlaceOfBirth\&quot;: {\n                            \&quot;Birthdate\&quot;: \&quot;2021-06-07\&quot;,\n                            \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                            \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                            \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                        }\n                    }, \n                    }\n                },\n                \&quot;SchemeName\&quot;: {\n                    \&quot;Proprietary\&quot;: \&quot;\&quot;\n                },\n                \&quot;PrivateIdentification\&quot;: {\n                    \&quot;DateAndPlaceOfBirth\&quot;: {\n                        \&quot;Birthdate\&quot;: \&quot;2021-05-28\&quot;,\n                        \&quot;ProvinceOfBirth\&quot;: \&quot;Uknown\&quot;,\n                        \&quot;CityOfBirth\&quot;: \&quot;\&quot;,\n                        \&quot;CountryOfBirth\&quot;: \&quot;ZAR\&quot;\n                    }\n                },\n                \&quot;ContactDetails\&quot;: {\n                    \&quot;MobileNumber\&quot;: \&quot;+277-23748020\&quot;\n                }\n            }\n        }\n    },\n    \&quot;SupplementaryData\&quot;: {\n        \&quot;payee.merchantClassificationCode\&quot;: \&quot;merchCode\&quot;,\n        \&quot;payer.merchantClassificationCode\&quot;: \&quot;merchCode\&quot;,\n        \&quot;transactionType.initiatorType\&quot;: \&quot;CONSUMER\&quot;,\n        \&quot;geoCode.latitude\&quot;: \&quot;string\&quot;,\n        \&quot;geoCode.longitude\&quot;: \&quot;string\&quot;\n    }\n}&quot;,
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
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}off-transaction-monitoring-service${Namespace}/monitor/transaction</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>ca4b30f2-59b0-4974-bb9f-51aef8f6a049</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable._meshednamespace</defaultValue>
      <description></description>
      <id>ded70d93-27a6-4ebe-a975-b9d495fc591f</id>
      <masked>false</masked>
      <name>Namespace</name>
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


println('checking if response code is 400 bad Request')
WS.verifyResponseStatusCode(response, 400)
assertThat(response.getStatusCode()).isEqualTo(400)
println('actual response code >> '+response)
 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
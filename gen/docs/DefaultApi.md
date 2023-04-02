# \DefaultApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clothes_get**](DefaultApi.md#clothes_get) | **get** /clothes | Retrieve a list of all clothing items
[**clothes_id_delete**](DefaultApi.md#clothes_id_delete) | **delete** /clothes/{id} | Delete an existing clothing item by ID
[**clothes_id_get**](DefaultApi.md#clothes_id_get) | **get** /clothes/{id} | Retrieve a single clothing item by ID
[**clothes_id_put**](DefaultApi.md#clothes_id_put) | **put** /clothes/{id} | Update an existing clothing item by ID
[**clothes_post**](DefaultApi.md#clothes_post) | **post** /clothes | Create a new clothing item
[**collations_get**](DefaultApi.md#collations_get) | **get** /collations | Retrieve a list of all collations
[**collations_id_delete**](DefaultApi.md#collations_id_delete) | **delete** /collations/{id} | Delete an existing collation by ID
[**collations_id_get**](DefaultApi.md#collations_id_get) | **get** /collations/{id} | Retrieve a single collation by ID
[**collations_id_put**](DefaultApi.md#collations_id_put) | **put** /collations/{id} | Update an existing collation by ID
[**collations_post**](DefaultApi.md#collations_post) | **post** /collations | Create a new collation
[**folders_get**](DefaultApi.md#folders_get) | **get** /folders | Retrieve a list of all folders
[**folders_id_delete**](DefaultApi.md#folders_id_delete) | **delete** /folders/{id} | Delete an existing folder by ID
[**folders_id_get**](DefaultApi.md#folders_id_get) | **get** /folders/{id} | Retrieve a single folder by ID
[**folders_id_put**](DefaultApi.md#folders_id_put) | **put** /folders/{id} | Update an existing folder by ID
[**folders_post**](DefaultApi.md#folders_post) | **post** /folders | Create a new folder



## clothes_get

> Vec<crate::models::Clothing> clothes_get()
Retrieve a list of all clothing items

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Clothing>**](Clothing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clothes_id_delete

> clothes_id_delete(id)
Delete an existing clothing item by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the clothing item to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clothes_id_get

> crate::models::Clothing clothes_id_get(id)
Retrieve a single clothing item by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the clothing item to retrieve | [required] |

### Return type

[**crate::models::Clothing**](Clothing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clothes_id_put

> crate::models::Clothing clothes_id_put(id, clothing)
Update an existing clothing item by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the clothing item to update | [required] |
**clothing** | [**Clothing**](Clothing.md) | Clothing object to be updated | [required] |

### Return type

[**crate::models::Clothing**](Clothing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clothes_post

> crate::models::Clothing clothes_post(clothing)
Create a new clothing item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clothing** | [**Clothing**](Clothing.md) | Clothing object to be created | [required] |

### Return type

[**crate::models::Clothing**](Clothing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collations_get

> Vec<crate::models::Collation> collations_get()
Retrieve a list of all collations

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Collation>**](Collation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collations_id_delete

> collations_id_delete(id)
Delete an existing collation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the collation to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collations_id_get

> crate::models::Collation collations_id_get(id)
Retrieve a single collation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the collation to retrieve | [required] |

### Return type

[**crate::models::Collation**](Collation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collations_id_put

> crate::models::Collation collations_id_put(id, collation)
Update an existing collation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the collation to update | [required] |
**collation** | [**Collation**](Collation.md) | Collation object to be updated | [required] |

### Return type

[**crate::models::Collation**](Collation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collations_post

> crate::models::Collation collations_post(collation)
Create a new collation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collation** | [**Collation**](Collation.md) | Collation object to be created | [required] |

### Return type

[**crate::models::Collation**](Collation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_get

> Vec<crate::models::Folder> folders_get()
Retrieve a list of all folders

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_id_delete

> folders_id_delete(id)
Delete an existing folder by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the folder to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_id_get

> crate::models::Folder folders_id_get(id)
Retrieve a single folder by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the folder to retrieve | [required] |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_id_put

> crate::models::Folder folders_id_put(id, folder)
Update an existing folder by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ID of the folder to update | [required] |
**folder** | [**Folder**](Folder.md) | Folder object to be updated | [required] |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_post

> crate::models::Folder folders_post(folder)
Create a new folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder** | [**Folder**](Folder.md) | Folder object to be created | [required] |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


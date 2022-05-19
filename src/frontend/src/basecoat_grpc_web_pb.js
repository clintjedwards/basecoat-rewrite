/**
 * @fileoverview gRPC-Web generated client stub for proto
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var basecoat_transport_pb = require('./basecoat_transport_pb.js')
const proto = {};
proto.proto = require('./basecoat_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.proto.BasecoatClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.proto.BasecoatPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.GetSystemInfoRequest,
 *   !proto.proto.GetSystemInfoResponse>}
 */
const methodDescriptor_Basecoat_GetSystemInfo = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/GetSystemInfo',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.GetSystemInfoRequest,
  basecoat_transport_pb.GetSystemInfoResponse,
  /**
   * @param {!proto.proto.GetSystemInfoRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.GetSystemInfoResponse.deserializeBinary
);


/**
 * @param {!proto.proto.GetSystemInfoRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.GetSystemInfoResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.GetSystemInfoResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.getSystemInfo =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/GetSystemInfo',
      request,
      metadata || {},
      methodDescriptor_Basecoat_GetSystemInfo,
      callback);
};


/**
 * @param {!proto.proto.GetSystemInfoRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.GetSystemInfoResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.getSystemInfo =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/GetSystemInfo',
      request,
      metadata || {},
      methodDescriptor_Basecoat_GetSystemInfo);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListOrganizationsRequest,
 *   !proto.proto.ListOrganizationsResponse>}
 */
const methodDescriptor_Basecoat_ListOrganizations = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListOrganizations',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListOrganizationsRequest,
  basecoat_transport_pb.ListOrganizationsResponse,
  /**
   * @param {!proto.proto.ListOrganizationsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListOrganizationsResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListOrganizationsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListOrganizationsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListOrganizationsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listOrganizations =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListOrganizations',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListOrganizations,
      callback);
};


/**
 * @param {!proto.proto.ListOrganizationsRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListOrganizationsResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listOrganizations =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListOrganizations',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListOrganizations);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateOrganizationRequest,
 *   !proto.proto.CreateOrganizationResponse>}
 */
const methodDescriptor_Basecoat_CreateOrganization = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateOrganization',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateOrganizationRequest,
  basecoat_transport_pb.CreateOrganizationResponse,
  /**
   * @param {!proto.proto.CreateOrganizationRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateOrganizationResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateOrganizationRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateOrganizationResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateOrganizationResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createOrganization =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateOrganization',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateOrganization,
      callback);
};


/**
 * @param {!proto.proto.CreateOrganizationRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateOrganizationResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createOrganization =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateOrganization',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateOrganization);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeOrganizationRequest,
 *   !proto.proto.DescribeOrganizationResponse>}
 */
const methodDescriptor_Basecoat_DescribeOrganization = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeOrganization',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeOrganizationRequest,
  basecoat_transport_pb.DescribeOrganizationResponse,
  /**
   * @param {!proto.proto.DescribeOrganizationRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeOrganizationResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeOrganizationRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeOrganizationResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeOrganizationResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeOrganization =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeOrganization',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeOrganization,
      callback);
};


/**
 * @param {!proto.proto.DescribeOrganizationRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeOrganizationResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeOrganization =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeOrganization',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeOrganization);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListUsersRequest,
 *   !proto.proto.ListUsersResponse>}
 */
const methodDescriptor_Basecoat_ListUsers = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListUsers',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListUsersRequest,
  basecoat_transport_pb.ListUsersResponse,
  /**
   * @param {!proto.proto.ListUsersRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListUsersResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListUsersRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListUsersResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListUsersResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listUsers =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListUsers',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListUsers,
      callback);
};


/**
 * @param {!proto.proto.ListUsersRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListUsersResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listUsers =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListUsers',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListUsers);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeUserRequest,
 *   !proto.proto.DescribeUserResponse>}
 */
const methodDescriptor_Basecoat_DescribeUser = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeUser',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeUserRequest,
  basecoat_transport_pb.DescribeUserResponse,
  /**
   * @param {!proto.proto.DescribeUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeUserResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeUserResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeUserResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeUser',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeUser,
      callback);
};


/**
 * @param {!proto.proto.DescribeUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeUserResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeUser',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateUserRequest,
 *   !proto.proto.CreateUserResponse>}
 */
const methodDescriptor_Basecoat_CreateUser = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateUser',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateUserRequest,
  basecoat_transport_pb.CreateUserResponse,
  /**
   * @param {!proto.proto.CreateUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateUserResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateUserResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateUserResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateUser',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateUser,
      callback);
};


/**
 * @param {!proto.proto.CreateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateUserResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateUser',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ResetUserPasswordRequest,
 *   !proto.proto.ResetUserPasswordResponse>}
 */
const methodDescriptor_Basecoat_ResetUserPassword = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ResetUserPassword',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ResetUserPasswordRequest,
  basecoat_transport_pb.ResetUserPasswordResponse,
  /**
   * @param {!proto.proto.ResetUserPasswordRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ResetUserPasswordResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ResetUserPasswordRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ResetUserPasswordResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ResetUserPasswordResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.resetUserPassword =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ResetUserPassword',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ResetUserPassword,
      callback);
};


/**
 * @param {!proto.proto.ResetUserPasswordRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ResetUserPasswordResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.resetUserPassword =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ResetUserPassword',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ResetUserPassword);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ToggleUserStateRequest,
 *   !proto.proto.ToggleUserStateResponse>}
 */
const methodDescriptor_Basecoat_ToggleUserState = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ToggleUserState',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ToggleUserStateRequest,
  basecoat_transport_pb.ToggleUserStateResponse,
  /**
   * @param {!proto.proto.ToggleUserStateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ToggleUserStateResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ToggleUserStateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ToggleUserStateResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ToggleUserStateResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.toggleUserState =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ToggleUserState',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ToggleUserState,
      callback);
};


/**
 * @param {!proto.proto.ToggleUserStateRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ToggleUserStateResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.toggleUserState =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ToggleUserState',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ToggleUserState);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListFormulasRequest,
 *   !proto.proto.ListFormulasResponse>}
 */
const methodDescriptor_Basecoat_ListFormulas = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListFormulas',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListFormulasRequest,
  basecoat_transport_pb.ListFormulasResponse,
  /**
   * @param {!proto.proto.ListFormulasRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListFormulasResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListFormulasRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListFormulasResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListFormulasResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listFormulas =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListFormulas',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListFormulas,
      callback);
};


/**
 * @param {!proto.proto.ListFormulasRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListFormulasResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listFormulas =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListFormulas',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListFormulas);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeFormulaRequest,
 *   !proto.proto.DescribeFormulaResponse>}
 */
const methodDescriptor_Basecoat_DescribeFormula = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeFormula',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeFormulaRequest,
  basecoat_transport_pb.DescribeFormulaResponse,
  /**
   * @param {!proto.proto.DescribeFormulaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeFormulaResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeFormulaResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeFormulaResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeFormula =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeFormula,
      callback);
};


/**
 * @param {!proto.proto.DescribeFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeFormulaResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeFormula =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeFormula);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.UpdateFormulaRequest,
 *   !proto.proto.UpdateFormulaResponse>}
 */
const methodDescriptor_Basecoat_UpdateFormula = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/UpdateFormula',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.UpdateFormulaRequest,
  basecoat_transport_pb.UpdateFormulaResponse,
  /**
   * @param {!proto.proto.UpdateFormulaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.UpdateFormulaResponse.deserializeBinary
);


/**
 * @param {!proto.proto.UpdateFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.UpdateFormulaResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.UpdateFormulaResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.updateFormula =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/UpdateFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateFormula,
      callback);
};


/**
 * @param {!proto.proto.UpdateFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.UpdateFormulaResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.updateFormula =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/UpdateFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateFormula);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateFormulaRequest,
 *   !proto.proto.CreateFormulaResponse>}
 */
const methodDescriptor_Basecoat_CreateFormula = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateFormula',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateFormulaRequest,
  basecoat_transport_pb.CreateFormulaResponse,
  /**
   * @param {!proto.proto.CreateFormulaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateFormulaResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateFormulaResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateFormulaResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createFormula =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateFormula,
      callback);
};


/**
 * @param {!proto.proto.CreateFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateFormulaResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createFormula =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateFormula);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DeleteFormulaRequest,
 *   !proto.proto.DeleteFormulaResponse>}
 */
const methodDescriptor_Basecoat_DeleteFormula = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DeleteFormula',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DeleteFormulaRequest,
  basecoat_transport_pb.DeleteFormulaResponse,
  /**
   * @param {!proto.proto.DeleteFormulaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DeleteFormulaResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DeleteFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DeleteFormulaResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DeleteFormulaResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.deleteFormula =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DeleteFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteFormula,
      callback);
};


/**
 * @param {!proto.proto.DeleteFormulaRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DeleteFormulaResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.deleteFormula =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DeleteFormula',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteFormula);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.AttachFormulaToJobRequest,
 *   !proto.proto.AttachFormulaToJobResponse>}
 */
const methodDescriptor_Basecoat_AttachFormulaToJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/AttachFormulaToJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.AttachFormulaToJobRequest,
  basecoat_transport_pb.AttachFormulaToJobResponse,
  /**
   * @param {!proto.proto.AttachFormulaToJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.AttachFormulaToJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.AttachFormulaToJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.AttachFormulaToJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.AttachFormulaToJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.attachFormulaToJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/AttachFormulaToJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_AttachFormulaToJob,
      callback);
};


/**
 * @param {!proto.proto.AttachFormulaToJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.AttachFormulaToJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.attachFormulaToJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/AttachFormulaToJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_AttachFormulaToJob);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DetachFormulaFromJobRequest,
 *   !proto.proto.DetachFormulaFromJobResponse>}
 */
const methodDescriptor_Basecoat_DetachFormulaFromJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DetachFormulaFromJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DetachFormulaFromJobRequest,
  basecoat_transport_pb.DetachFormulaFromJobResponse,
  /**
   * @param {!proto.proto.DetachFormulaFromJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DetachFormulaFromJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DetachFormulaFromJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DetachFormulaFromJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DetachFormulaFromJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.detachFormulaFromJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DetachFormulaFromJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DetachFormulaFromJob,
      callback);
};


/**
 * @param {!proto.proto.DetachFormulaFromJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DetachFormulaFromJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.detachFormulaFromJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DetachFormulaFromJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DetachFormulaFromJob);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListColorantsRequest,
 *   !proto.proto.ListColorantsResponse>}
 */
const methodDescriptor_Basecoat_ListColorants = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListColorants',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListColorantsRequest,
  basecoat_transport_pb.ListColorantsResponse,
  /**
   * @param {!proto.proto.ListColorantsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListColorantsResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListColorantsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListColorantsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListColorantsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listColorants =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListColorants',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListColorants,
      callback);
};


/**
 * @param {!proto.proto.ListColorantsRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListColorantsResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listColorants =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListColorants',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListColorants);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeColorantRequest,
 *   !proto.proto.DescribeColorantResponse>}
 */
const methodDescriptor_Basecoat_DescribeColorant = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeColorant',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeColorantRequest,
  basecoat_transport_pb.DescribeColorantResponse,
  /**
   * @param {!proto.proto.DescribeColorantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeColorantResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeColorantResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeColorantResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeColorant =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeColorant,
      callback);
};


/**
 * @param {!proto.proto.DescribeColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeColorantResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeColorant =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeColorant);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateColorantRequest,
 *   !proto.proto.CreateColorantResponse>}
 */
const methodDescriptor_Basecoat_CreateColorant = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateColorant',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateColorantRequest,
  basecoat_transport_pb.CreateColorantResponse,
  /**
   * @param {!proto.proto.CreateColorantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateColorantResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateColorantResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateColorantResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createColorant =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateColorant,
      callback);
};


/**
 * @param {!proto.proto.CreateColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateColorantResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createColorant =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateColorant);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DeleteColorantRequest,
 *   !proto.proto.DeleteColorantResponse>}
 */
const methodDescriptor_Basecoat_DeleteColorant = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DeleteColorant',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DeleteColorantRequest,
  basecoat_transport_pb.DeleteColorantResponse,
  /**
   * @param {!proto.proto.DeleteColorantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DeleteColorantResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DeleteColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DeleteColorantResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DeleteColorantResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.deleteColorant =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DeleteColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteColorant,
      callback);
};


/**
 * @param {!proto.proto.DeleteColorantRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DeleteColorantResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.deleteColorant =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DeleteColorant',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteColorant);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListBasesRequest,
 *   !proto.proto.ListBasesResponse>}
 */
const methodDescriptor_Basecoat_ListBases = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListBases',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListBasesRequest,
  basecoat_transport_pb.ListBasesResponse,
  /**
   * @param {!proto.proto.ListBasesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListBasesResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListBasesRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListBasesResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListBasesResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listBases =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListBases',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListBases,
      callback);
};


/**
 * @param {!proto.proto.ListBasesRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListBasesResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listBases =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListBases',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListBases);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeBaseRequest,
 *   !proto.proto.DescribeBaseResponse>}
 */
const methodDescriptor_Basecoat_DescribeBase = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeBase',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeBaseRequest,
  basecoat_transport_pb.DescribeBaseResponse,
  /**
   * @param {!proto.proto.DescribeBaseRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeBaseResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeBaseResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeBaseResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeBase =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeBase,
      callback);
};


/**
 * @param {!proto.proto.DescribeBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeBaseResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeBase =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeBase);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateBaseRequest,
 *   !proto.proto.CreateBaseResponse>}
 */
const methodDescriptor_Basecoat_CreateBase = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateBase',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateBaseRequest,
  basecoat_transport_pb.CreateBaseResponse,
  /**
   * @param {!proto.proto.CreateBaseRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateBaseResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateBaseResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateBaseResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createBase =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateBase,
      callback);
};


/**
 * @param {!proto.proto.CreateBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateBaseResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createBase =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateBase);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DeleteBaseRequest,
 *   !proto.proto.DeleteBaseResponse>}
 */
const methodDescriptor_Basecoat_DeleteBase = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DeleteBase',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DeleteBaseRequest,
  basecoat_transport_pb.DeleteBaseResponse,
  /**
   * @param {!proto.proto.DeleteBaseRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DeleteBaseResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DeleteBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DeleteBaseResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DeleteBaseResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.deleteBase =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DeleteBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteBase,
      callback);
};


/**
 * @param {!proto.proto.DeleteBaseRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DeleteBaseResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.deleteBase =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DeleteBase',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteBase);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListContractorsRequest,
 *   !proto.proto.ListContractorsResponse>}
 */
const methodDescriptor_Basecoat_ListContractors = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListContractors',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListContractorsRequest,
  basecoat_transport_pb.ListContractorsResponse,
  /**
   * @param {!proto.proto.ListContractorsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListContractorsResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListContractorsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListContractorsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListContractorsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listContractors =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListContractors',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListContractors,
      callback);
};


/**
 * @param {!proto.proto.ListContractorsRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListContractorsResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listContractors =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListContractors',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListContractors);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeContractorRequest,
 *   !proto.proto.DescribeContractorResponse>}
 */
const methodDescriptor_Basecoat_DescribeContractor = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeContractor',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeContractorRequest,
  basecoat_transport_pb.DescribeContractorResponse,
  /**
   * @param {!proto.proto.DescribeContractorRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeContractorResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeContractorResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeContractorResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeContractor =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeContractor,
      callback);
};


/**
 * @param {!proto.proto.DescribeContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeContractorResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeContractor =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeContractor);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.UpdateContractorRequest,
 *   !proto.proto.UpdateContractorResponse>}
 */
const methodDescriptor_Basecoat_UpdateContractor = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/UpdateContractor',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.UpdateContractorRequest,
  basecoat_transport_pb.UpdateContractorResponse,
  /**
   * @param {!proto.proto.UpdateContractorRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.UpdateContractorResponse.deserializeBinary
);


/**
 * @param {!proto.proto.UpdateContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.UpdateContractorResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.UpdateContractorResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.updateContractor =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/UpdateContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateContractor,
      callback);
};


/**
 * @param {!proto.proto.UpdateContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.UpdateContractorResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.updateContractor =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/UpdateContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateContractor);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateContractorRequest,
 *   !proto.proto.CreateContractorResponse>}
 */
const methodDescriptor_Basecoat_CreateContractor = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateContractor',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateContractorRequest,
  basecoat_transport_pb.CreateContractorResponse,
  /**
   * @param {!proto.proto.CreateContractorRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateContractorResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateContractorResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateContractorResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createContractor =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateContractor,
      callback);
};


/**
 * @param {!proto.proto.CreateContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateContractorResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createContractor =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateContractor);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DeleteContractorRequest,
 *   !proto.proto.DeleteContractorResponse>}
 */
const methodDescriptor_Basecoat_DeleteContractor = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DeleteContractor',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DeleteContractorRequest,
  basecoat_transport_pb.DeleteContractorResponse,
  /**
   * @param {!proto.proto.DeleteContractorRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DeleteContractorResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DeleteContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DeleteContractorResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DeleteContractorResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.deleteContractor =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DeleteContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteContractor,
      callback);
};


/**
 * @param {!proto.proto.DeleteContractorRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DeleteContractorResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.deleteContractor =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DeleteContractor',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteContractor);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.ListJobsRequest,
 *   !proto.proto.ListJobsResponse>}
 */
const methodDescriptor_Basecoat_ListJobs = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/ListJobs',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.ListJobsRequest,
  basecoat_transport_pb.ListJobsResponse,
  /**
   * @param {!proto.proto.ListJobsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.ListJobsResponse.deserializeBinary
);


/**
 * @param {!proto.proto.ListJobsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.ListJobsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.ListJobsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.listJobs =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/ListJobs',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListJobs,
      callback);
};


/**
 * @param {!proto.proto.ListJobsRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.ListJobsResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.listJobs =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/ListJobs',
      request,
      metadata || {},
      methodDescriptor_Basecoat_ListJobs);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DescribeJobRequest,
 *   !proto.proto.DescribeJobResponse>}
 */
const methodDescriptor_Basecoat_DescribeJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DescribeJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DescribeJobRequest,
  basecoat_transport_pb.DescribeJobResponse,
  /**
   * @param {!proto.proto.DescribeJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DescribeJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DescribeJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DescribeJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DescribeJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.describeJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DescribeJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeJob,
      callback);
};


/**
 * @param {!proto.proto.DescribeJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DescribeJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.describeJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DescribeJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DescribeJob);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.UpdateJobRequest,
 *   !proto.proto.UpdateJobResponse>}
 */
const methodDescriptor_Basecoat_UpdateJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/UpdateJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.UpdateJobRequest,
  basecoat_transport_pb.UpdateJobResponse,
  /**
   * @param {!proto.proto.UpdateJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.UpdateJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.UpdateJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.UpdateJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.UpdateJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.updateJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/UpdateJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateJob,
      callback);
};


/**
 * @param {!proto.proto.UpdateJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.UpdateJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.updateJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/UpdateJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_UpdateJob);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.CreateJobRequest,
 *   !proto.proto.CreateJobResponse>}
 */
const methodDescriptor_Basecoat_CreateJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/CreateJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.CreateJobRequest,
  basecoat_transport_pb.CreateJobResponse,
  /**
   * @param {!proto.proto.CreateJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.CreateJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.CreateJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.CreateJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.CreateJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.createJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/CreateJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateJob,
      callback);
};


/**
 * @param {!proto.proto.CreateJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.CreateJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.createJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/CreateJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_CreateJob);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.proto.DeleteJobRequest,
 *   !proto.proto.DeleteJobResponse>}
 */
const methodDescriptor_Basecoat_DeleteJob = new grpc.web.MethodDescriptor(
  '/proto.Basecoat/DeleteJob',
  grpc.web.MethodType.UNARY,
  basecoat_transport_pb.DeleteJobRequest,
  basecoat_transport_pb.DeleteJobResponse,
  /**
   * @param {!proto.proto.DeleteJobRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  basecoat_transport_pb.DeleteJobResponse.deserializeBinary
);


/**
 * @param {!proto.proto.DeleteJobRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.proto.DeleteJobResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.proto.DeleteJobResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.proto.BasecoatClient.prototype.deleteJob =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/proto.Basecoat/DeleteJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteJob,
      callback);
};


/**
 * @param {!proto.proto.DeleteJobRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.proto.DeleteJobResponse>}
 *     Promise that resolves to the response
 */
proto.proto.BasecoatPromiseClient.prototype.deleteJob =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/proto.Basecoat/DeleteJob',
      request,
      metadata || {},
      methodDescriptor_Basecoat_DeleteJob);
};


module.exports = proto.proto;


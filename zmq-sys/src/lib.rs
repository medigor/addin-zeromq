extern crate libc;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use crate::unix::RawFd;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::RawFd;

pub mod errno;

pub use crate::ffi::{
    // These are the non-deprecated constants defined in zmq.h. Note that this
    // list exceeds what is defined by the current minimum required version of
    // libzmq, but is instead based on the latest stable release. The idea here
    // is to expose newer API, when it is still compatible with the ABI defined
    // by the libzmq library. For example, using new socket options will just
    // result in a runtime error when used with an older libzmq, but
    // ABI-compatible libzmq, which does not know about them.
    ZMQ_HAUSNUMERO,
    ZMQ_IO_THREADS,
    ZMQ_MAX_SOCKETS,
    ZMQ_SOCKET_LIMIT,
    ZMQ_THREAD_PRIORITY,
    ZMQ_THREAD_SCHED_POLICY,
    ZMQ_MAX_MSGSZ,
    ZMQ_MSG_T_SIZE,
    ZMQ_THREAD_AFFINITY_CPU_ADD,
    ZMQ_THREAD_AFFINITY_CPU_REMOVE,
    ZMQ_THREAD_NAME_PREFIX,
    ZMQ_IO_THREADS_DFLT,
    ZMQ_MAX_SOCKETS_DFLT,
    ZMQ_THREAD_PRIORITY_DFLT,
    ZMQ_THREAD_SCHED_POLICY_DFLT,
    ZMQ_PAIR,
    ZMQ_PUB,
    ZMQ_SUB,
    ZMQ_REQ,
    ZMQ_REP,
    ZMQ_DEALER,
    ZMQ_ROUTER,
    ZMQ_PULL,
    ZMQ_PUSH,
    ZMQ_XPUB,
    ZMQ_XSUB,
    ZMQ_STREAM,
    ZMQ_AFFINITY,
    ZMQ_ROUTING_ID,
    ZMQ_SUBSCRIBE,
    ZMQ_UNSUBSCRIBE,
    ZMQ_RATE,
    ZMQ_RECOVERY_IVL,
    ZMQ_SNDBUF,
    ZMQ_RCVBUF,
    ZMQ_RCVMORE,
    ZMQ_FD,
    ZMQ_EVENTS,
    ZMQ_TYPE,
    ZMQ_LINGER,
    ZMQ_RECONNECT_IVL,
    ZMQ_BACKLOG,
    ZMQ_RECONNECT_IVL_MAX,
    ZMQ_MAXMSGSIZE,
    ZMQ_SNDHWM,
    ZMQ_RCVHWM,
    ZMQ_MULTICAST_HOPS,
    ZMQ_RCVTIMEO,
    ZMQ_SNDTIMEO,
    ZMQ_LAST_ENDPOINT,
    ZMQ_ROUTER_MANDATORY,
    ZMQ_TCP_KEEPALIVE,
    ZMQ_TCP_KEEPALIVE_CNT,
    ZMQ_TCP_KEEPALIVE_IDLE,
    ZMQ_TCP_KEEPALIVE_INTVL,
    ZMQ_IMMEDIATE,
    ZMQ_XPUB_VERBOSE,
    ZMQ_ROUTER_RAW,
    ZMQ_IPV6,
    ZMQ_MECHANISM,
    ZMQ_PLAIN_SERVER,
    ZMQ_PLAIN_USERNAME,
    ZMQ_PLAIN_PASSWORD,
    ZMQ_CURVE_SERVER,
    ZMQ_CURVE_PUBLICKEY,
    ZMQ_CURVE_SECRETKEY,
    ZMQ_CURVE_SERVERKEY,
    ZMQ_PROBE_ROUTER,
    ZMQ_REQ_CORRELATE,
    ZMQ_REQ_RELAXED,
    ZMQ_CONFLATE,
    ZMQ_ZAP_DOMAIN,
    ZMQ_ROUTER_HANDOVER,
    ZMQ_TOS,
    ZMQ_CONNECT_ROUTING_ID,
    ZMQ_GSSAPI_SERVER,
    ZMQ_GSSAPI_PRINCIPAL,
    ZMQ_GSSAPI_SERVICE_PRINCIPAL,
    ZMQ_GSSAPI_PLAINTEXT,
    ZMQ_HANDSHAKE_IVL,
    ZMQ_SOCKS_PROXY,
    ZMQ_XPUB_NODROP,
    ZMQ_BLOCKY,
    ZMQ_XPUB_MANUAL,
    ZMQ_XPUB_WELCOME_MSG,
    ZMQ_STREAM_NOTIFY,
    ZMQ_INVERT_MATCHING,
    ZMQ_HEARTBEAT_IVL,
    ZMQ_HEARTBEAT_TTL,
    ZMQ_HEARTBEAT_TIMEOUT,
    ZMQ_XPUB_VERBOSER,
    ZMQ_CONNECT_TIMEOUT,
    ZMQ_TCP_MAXRT,
    ZMQ_THREAD_SAFE,
    ZMQ_MULTICAST_MAXTPDU,
    ZMQ_VMCI_BUFFER_SIZE,
    ZMQ_VMCI_BUFFER_MIN_SIZE,
    ZMQ_VMCI_BUFFER_MAX_SIZE,
    ZMQ_VMCI_CONNECT_TIMEOUT,
    ZMQ_USE_FD,
    ZMQ_GSSAPI_PRINCIPAL_NAMETYPE,
    ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE,
    ZMQ_BINDTODEVICE,
    ZMQ_MORE,
    ZMQ_SHARED,
    ZMQ_DONTWAIT,
    ZMQ_SNDMORE,
    ZMQ_NULL,
    ZMQ_PLAIN,
    ZMQ_CURVE,
    ZMQ_GSSAPI,
    ZMQ_GROUP_MAX_LENGTH,
    ZMQ_EVENT_CONNECTED,
    ZMQ_EVENT_CONNECT_DELAYED,
    ZMQ_EVENT_CONNECT_RETRIED,
    ZMQ_EVENT_LISTENING,
    ZMQ_EVENT_BIND_FAILED,
    ZMQ_EVENT_ACCEPTED,
    ZMQ_EVENT_ACCEPT_FAILED,
    ZMQ_EVENT_CLOSED,
    ZMQ_EVENT_CLOSE_FAILED,
    ZMQ_EVENT_DISCONNECTED,
    ZMQ_EVENT_MONITOR_STOPPED,
    ZMQ_EVENT_ALL,
    ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL,
    ZMQ_EVENT_HANDSHAKE_SUCCEEDED,
    ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL,
    ZMQ_EVENT_HANDSHAKE_FAILED_AUTH,
    ZMQ_PROTOCOL_ERROR_ZMTP_UNSPECIFIED,
    ZMQ_PROTOCOL_ERROR_ZMTP_UNEXPECTED_COMMAND,
    ZMQ_PROTOCOL_ERROR_ZMTP_INVALID_SEQUENCE,
    ZMQ_PROTOCOL_ERROR_ZMTP_KEY_EXCHANGE,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_UNSPECIFIED,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_MESSAGE,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_HELLO,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_INITIATE,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_ERROR,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_READY,
    ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_WELCOME,
    ZMQ_PROTOCOL_ERROR_ZMTP_INVALID_METADATA,
    ZMQ_PROTOCOL_ERROR_ZMTP_CRYPTOGRAPHIC,
    ZMQ_PROTOCOL_ERROR_ZMTP_MECHANISM_MISMATCH,
    ZMQ_PROTOCOL_ERROR_ZAP_UNSPECIFIED,
    ZMQ_PROTOCOL_ERROR_ZAP_MALFORMED_REPLY,
    ZMQ_PROTOCOL_ERROR_ZAP_BAD_REQUEST_ID,
    ZMQ_PROTOCOL_ERROR_ZAP_BAD_VERSION,
    ZMQ_PROTOCOL_ERROR_ZAP_INVALID_STATUS_CODE,
    ZMQ_PROTOCOL_ERROR_ZAP_INVALID_METADATA,

    // These are "deprecated" in favor of the `zmq_poller` API, but are still
    // used by `zmq`, and the `zmq_poller` API has been introduced only in
    // 4.2.0, and have not been stabilized at least up until 4.3.1.
    ZMQ_POLLIN,
    ZMQ_POLLOUT,
    ZMQ_POLLERR,
    ZMQ_POLLPRI,

    ZMQ_POLLITEMS_DFLT,

    // Undeprecated types.
    zmq_msg_t,
    zmq_free_fn,

    // Undeprecated and documented functions (or more generally, symbols). These
    // must exist in the ABI of the oldest supported libzmq version, so
    // extending this list requires bumping that.
    zmq_pollitem_t,
    zmq_version,
    zmq_errno,
    zmq_strerror,
    zmq_ctx_new,
    zmq_ctx_term,
    zmq_ctx_shutdown,
    zmq_ctx_set,
    zmq_ctx_get,
    zmq_msg_init,
    zmq_msg_init_size,
    zmq_msg_init_data,
    zmq_msg_send,
    zmq_msg_recv,
    zmq_msg_close,
    zmq_msg_move,
    zmq_msg_copy,
    zmq_msg_data,
    zmq_msg_size,
    zmq_msg_more,
    zmq_msg_get,
    zmq_msg_set,
    zmq_msg_gets,
    zmq_socket,
    zmq_close,
    zmq_setsockopt,
    zmq_getsockopt,
    zmq_bind,
    zmq_connect,
    zmq_unbind,
    zmq_disconnect,
    zmq_send,
    zmq_send_const,
    zmq_recv,
    zmq_socket_monitor,
    zmq_sendmsg,
    zmq_recvmsg,
    zmq_sendiov,
    zmq_recviov,
    zmq_poll,
    zmq_proxy,
    zmq_proxy_steerable,
    zmq_has,
    zmq_device,
    zmq_z85_encode,
    zmq_z85_decode,
    zmq_curve_keypair,
};

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(clippy::unreadable_literal)]
mod ffi;
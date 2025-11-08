set(_app_executable ${CMAKE_SOURCE_DIR}/target/debug/xdata-app)

# Cannot make output dep because we don't know the src changed.
add_custom_target(xdata-app
    COMMAND cargo build --package xdata-app
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building xdata-app with Cargo"
    VERBATIM
)

add_custom_target(xdata-app-image
    COMMAND docker build -t xdata-app
        -f apps/xdata-app/deploy/Dockerfile .
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    DEPENDS xdata-app
)

add_custom_target(xdata-deploy
    COMMAND kubectl apply -f apps/xdata-app/deploy/statefulset.yaml
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_custom_target(xdata-delete
    COMMAND kubectl delete -f apps/xdata-app/deploy/statefulset.yaml
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)
use axum::{Extension, Json,http::StatusCode,response::IntoResponse,extract::Path};


use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;
use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS,
};

pub async fn product_all(Extension(pool): Extension<PgPool>)->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>  {
    let sql = "SELECT * FROM products ".to_string();

    let task = sqlx::query_as::<_, models::auth::Product>(&sql).fetch_all(&pool).await.unwrap();
   if task.is_empty(){
        let error_response = serde_json::json!({
            "status": "fail",
            "message" : "Something bad happended ",
        });

        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    let json_response = serde_json::json!({
        "status": "success",
        "results": task.len(),
        "Products": task
    });
    Ok(Json(json_response))
}

 pub async fn product_byid(Path(id): Path<String>,Extension(pool): Extension<PgPool>)
 ->Result<Json<Value>, AppError>{
   
    let sql = "SELECT * FROM products where id=$1".to_string();

    let product = sqlx::query_as::<_, models::auth::Product>(
        "SELECT * FROM products where id=$1 ",
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

   
    let json_response=serde_json::json!({
        "status":"success",
        "results":"oke",
        "notes":product
    });
    Ok(Json(json_response))

}

pub async fn product_create(
    Json(product): Json<models::auth::Product>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if product.name.is_empty() || product.price.is_empty() || product.description.is_empty() {
        return Err(AppError::ProductNotFound)
    }
    let sql="INSERT INTO products (id ,name,description,quantity,price) VALUES ($1,$2,$3,$4,$5)";
    let _ =sqlx::query(&sql)
        .bind(&product.id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(&product.quantity)
        .bind(&product.price)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;   

        Ok(Json(json!({ "msg": "Product successfully" })))

}
pub async fn product_delete(Path(id): Path<String>,Extension(pool): Extension<PgPool>)
->Result<Json<Value>, AppError>{
   
    sqlx::query("DELETE FROM products WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?;
    Ok(Json(json!({"msg":"Product deleted"})))


}

pub async fn login(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    // get the user for the email from database
    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT * FROM productusers where email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
        //if user exits then:

        // if password is encrypted than decode it first before comparing
        if user.password != credentials.password {
            // password is incorrect
            Err(AppError::WrongCredential)
        } else {
            let claims = Claims {
                email: credentials.email.to_owned(),
                exp: get_timestamp_8_hours_from_now(),
            };
            let token = encode(&Header::default(), &claims, &KEYS.encoding)
                .map_err(|_| AppError::TokenCreation)?;
            // return bearer token
            Ok(Json(json!({ "access_token": token, "type": "Bearer" })))
        }
    } else {
        // if the user does not exit
        Err(AppError::UserDoesNotExist)
    }
}


pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    // get the user for the email from database
    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT * FROM ProductUsers where email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = user {
        //if a user with email already exits send error
        return Err(AppError::UserAlreadyExits);
    }
    
    let result = sqlx::query("INSERT INTO ProductUsers (id,email, name,password) values ($1, $2,$3,$4)")
        .bind(&credentials.id)
        .bind(&credentials.email)
        .bind(&credentials.name)
        .bind(&credentials.password)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "msg": "registered successfully" })))
    }

}
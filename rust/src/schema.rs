// @generated automatically by Diesel CLI.

diesel::table! {
    stockdata (unixtime) {
        ticker -> Text,
        unixtime -> Nullable<Integer>,
        currentprice -> Float,
        high -> Nullable<Float>,
        low -> Nullable<Float>,
        opening -> Nullable<Float>,
        pclose -> Nullable<Float>,
    }
}

package io.nordnotes.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;

@Getter
public class ResultDto<T> {

  /** Data payload. */
  @JsonProperty("data")
  private T data;

  /**
   * Creates new data payload.
   *
   * @param data Payload.
   */
  public ResultDto(T data) {
    this.data = data;
  }
}
